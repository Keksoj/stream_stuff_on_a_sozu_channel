// This is Sōzu's code as of the 0.14 dev branch (2021-12-29)
// but the error! has been replaced with println! for convenience
use std::{
    cmp::min,
    fmt::Debug,
    io::{self, ErrorKind, Read, Write},
    iter::Iterator,
    marker::PhantomData,
    os::unix::{
        self,
        io::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
        net,
    },
    str::from_utf8,
    time::Duration,
};

use anyhow::Context;
use mio::{event::Source, net::UnixStream};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use crate::copy_pasted_from_sozu::{buffer::growable::Buffer, ready::Ready};

#[derive(Debug, PartialEq)]
pub enum ConnError {
    Continue,
    ParseError,
    SocketError,
}

#[derive(Debug)]
pub struct Channel<Tx, Rx> {
    pub sock: UnixStream,
    front_buf: Buffer,
    pub back_buf: Buffer,
    max_buffer_size: usize,
    pub readiness: Ready,
    pub interest: Ready,
    blocking: bool,
    phantom_tx: PhantomData<Tx>,
    phantom_rx: PhantomData<Rx>,
}

impl<Tx: Debug + Serialize, Rx: Debug + DeserializeOwned> Channel<Tx, Rx> {
    pub fn from_path(
        path: &str,
        buffer_size: usize,
        max_buffer_size: usize,
    ) -> anyhow::Result<Channel<Tx, Rx>> {
        let unix_stream = UnixStream::connect(path).context("Can not connect to unix socket")?;
        let channel = Channel::new(unix_stream, buffer_size, max_buffer_size);
        Ok(channel)
    }

    pub fn new(sock: UnixStream, buffer_size: usize, max_buffer_size: usize) -> Channel<Tx, Rx> {
        Channel {
            sock,
            front_buf: Buffer::with_capacity(buffer_size),
            back_buf: Buffer::with_capacity(buffer_size),
            max_buffer_size,
            readiness: Ready::empty(),
            interest: Ready::readable(),
            blocking: false,
            phantom_tx: PhantomData,
            phantom_rx: PhantomData,
        }
    }

    pub fn into<Tx2: Debug + Serialize, Rx2: Debug + DeserializeOwned>(self) -> Channel<Tx2, Rx2> {
        Channel {
            sock: self.sock,
            front_buf: self.front_buf,
            back_buf: self.back_buf,
            max_buffer_size: self.max_buffer_size,
            readiness: self.readiness,
            interest: self.interest,
            blocking: self.blocking,
            phantom_tx: PhantomData,
            phantom_rx: PhantomData,
        }
    }

    // this uses the standard lib's UnixStream
    pub fn set_nonblocking(&mut self, nonblocking: bool) {
        unsafe {
            // get the raw file descriptor of the channel's socket
            let fd = self.sock.as_raw_fd();

            // create a unixstream from this raw file descriptor, essentially wrapping it
            // THIS IS UNSAFE
            // this consumes the file descriptor
            let stream = unix::net::UnixStream::from_raw_fd(fd);

            // the standard library does unix magic by wrapping the libc:
            // unsafe {
            //     let v = nonblocking as c_int;
            //     cvt(libc::ioctl(self.as_raw_fd(), libc::FIONBIO, &v))?;
            //     Ok(())
            // }
            // it unsafe but wrapped into a result so we can map the error
            let _ = stream.set_nonblocking(nonblocking).map_err(|e| {
                println!(
                    "error: could not change blocking status for stream: {:?}",
                    e
                );
            });

            // recreate the file descriptor from the socket we changed.
            // but why is it unused then?
            let _fd = stream.into_raw_fd();
        }
        self.blocking = !nonblocking;
        println!(
            "Set channel to {}",
            match self.blocking {
                true => "blocking",
                false => "nonblocking",
            }
        );
    }

    pub fn set_blocking(&mut self, blocking: bool) {
        self.set_nonblocking(!blocking)
    }

    pub fn fd(&self) -> RawFd {
        self.sock.as_raw_fd()
    }

    pub fn handle_events(&mut self, events: Ready) {
        self.readiness |= events;
    }

    pub fn readiness(&self) -> Ready {
        self.readiness & self.interest
    }

    // this is used in sozu_lib only
    pub fn run(&mut self) {
        let interest = self.interest & self.readiness;

        if interest.is_readable() {
            let _ = self.readable().map_err(|e| {
                println!("error: error reading from channel: {:?}", e);
            });
        }

        if interest.is_writable() {
            let _ = self.writable().map_err(|e| {
                println!("error: error writing to channel: {:?}", e);
            });
        }
    }

    pub fn readable(&mut self) -> Result<usize, ConnError> {
        if !(self.interest & self.readiness).is_readable() {
            return Err(ConnError::Continue);
        }

        let mut count = 0usize;
        loop {
            let size = self.front_buf.available_space();
            if size == 0 {
                self.interest.remove(Ready::readable());
                break;
            }

            match self.sock.read(self.front_buf.space()) {
                Ok(0) => {
                    self.interest = Ready::empty();
                    self.readiness.remove(Ready::readable());
                    println!("error: read() returned 0 (count={})", count);
                    self.readiness.insert(Ready::hup());
                    return Err(ConnError::SocketError);
                }
                Err(e) => {
                    match e.kind() {
                        ErrorKind::WouldBlock => {
                            self.readiness.remove(Ready::readable());
                            break;
                        }
                        _ => {
                            //log!(log::LogLevel::Error, "UNIX CLIENT[{}] read error (kind: {:?}): {:?}", tok.0, code, e);
                            self.interest = Ready::empty();
                            self.readiness = Ready::empty();
                            return Err(ConnError::SocketError);
                        }
                    }
                }
                Ok(r) => {
                    count += r;
                    self.front_buf.fill(r);
                    //debug!("UNIX CLIENT[{}] sent {} bytes: {:?}", tok.0, r, from_utf8(self.buf.data()));
                }
            };
        }

        Ok(count)
    }

    pub fn writable(&mut self) -> Result<usize, ConnError> {
        if !(self.interest & self.readiness).is_writable() {
            return Err(ConnError::Continue);
        }

        let mut count = 0usize;
        loop {
            let size = self.back_buf.available_data();
            if size == 0 {
                self.interest.remove(Ready::writable());
                break;
            }

            match self.sock.write(self.back_buf.data()) {
                Ok(0) => {
                    //println!("error: write() returned 0");
                    self.interest = Ready::empty();
                    self.readiness.insert(Ready::hup());
                    return Err(ConnError::SocketError);
                }
                Ok(r) => {
                    count += r;
                    self.back_buf.consume(r);
                }
                Err(e) => match e.kind() {
                    ErrorKind::WouldBlock => {
                        self.readiness.remove(Ready::writable());
                        break;
                    }
                    e => {
                        println!("error: channel write error: {:?}", e);
                        self.interest = Ready::empty();
                        self.readiness = Ready::empty();
                        return Err(ConnError::SocketError);
                    }
                },
            }
        }

        Ok(count)
    }

    pub fn read_message(&mut self) -> Option<Rx> {
        if self.blocking {
            println!("reading blocking");
            self.read_message_blocking()
        } else {
            println!("reading nonblocking");
            self.read_message_nonblocking()
        }
    }

    pub fn read_message_nonblocking(&mut self) -> Option<Rx> {
        if let Some(pos) = self.front_buf.data().iter().position(|&x| x == 0) {
            let mut res = None;

            println!("Reading buffer, converting to utf8");

            if let Ok(s) = from_utf8(&self.front_buf.data()[..pos]) {
                println!("parsing utf8 to json");

                match serde_json::from_str(s) {
                    Ok(message) => res = Some(message),
                    Err(e) => println!(
                        "error: could not parse message (error={:?}), ignoring:\n{}",
                        e, s
                    ),
                }
            } else {
                println!("error: invalid utf-8 encoding in command message, ignoring");
            }

            self.front_buf.consume(pos + 1);
            res
        } else {
            if self.front_buf.available_space() == 0 {
                if self.front_buf.capacity() == self.max_buffer_size {
                    println!("error: command buffer full, cannot grow more, ignoring");
                } else {
                    let new_size = min(self.front_buf.capacity() + 5000, self.max_buffer_size);
                    self.front_buf.grow(new_size);
                }
            }

            self.interest.insert(Ready::readable());
            None
        }
    }

    pub fn read_message_blocking(&mut self) -> Option<Rx> {
        self.read_message_blocking_timeout(None)
    }

    pub fn read_message_blocking_timeout(&mut self, timeout: Option<Duration>) -> Option<Rx> {
        println!("timeout: {:?}", timeout);

        let now = std::time::Instant::now();

        loop {
            println!("start reading loop");
            if timeout.is_some() && now.elapsed() >= timeout.unwrap() {
                println!("time is out!");
                return None;
            }

            if let Some(pos) = self.front_buf.data().iter().position(|&x| x == 0) {
                let mut res = None;
                println!("Reading buffer, converting to utf8");

                if let Ok(s) = from_utf8(&self.front_buf.data()[..pos]) {
                    println!("parsing utf8 to json");
                    match serde_json::from_str(s) {
                        Ok(message) => res = Some(message),
                        Err(e) => {
                            println!(
                                "error: could not parse message (error={:?}), ignoring:\n{}",
                                e, s
                            )
                        }
                    }
                } else {
                    println!("error: invalid utf-8 encoding in command message, ignoring");
                }

                self.front_buf.consume(pos + 1);
                return res;
            } else {
                println!("Reading but not from the buffer, from the socket, blocking it");
                if self.front_buf.available_space() == 0 {
                    if self.front_buf.capacity() == self.max_buffer_size {
                        println!("error: command buffer full, cannot grow more, ignoring");
                        return None;
                    } else {
                        let new_size = min(self.front_buf.capacity() + 5000, self.max_buffer_size);
                        self.front_buf.grow(new_size);
                    }
                }

                println!("Reading from the unix stream into the channel buffer");
                match self
                    .sock
                    // things are blocking here!
                    .read(self.front_buf.space())
                {
                    Ok(0) => {
                        println!("Nothing to read");
                        return None;
                    }
                    Err(e) => {
                        println!("Something went wrong: {}", e);
                        return None;
                    }
                    Ok(r) => {
                        println!(
                            "Read {} bytes, filling the front buffer with just as many",
                            r
                        );
                        self.front_buf.fill(r);
                    }
                };
            }
        }
    }

    pub fn write_message(&mut self, message: &Tx) -> bool {
        if self.blocking {
            println!("Writing message, blocking");
            self.write_message_blocking(message)
        } else {
            println!("Writing message, nonblocking");
            self.write_message_nonblocking(message)
        }
    }

    pub fn write_message_nonblocking(&mut self, message: &Tx) -> bool {
        println!("converting to bytes");
        let message = &serde_json::to_string(message)
            .map(|s| s.into_bytes())
            .unwrap_or_else(|_| Vec::new());

        let msg_len = message.len() + 1;

        if msg_len > self.back_buf.available_space() {
            println!("shifting the back buffer");
            self.back_buf.shift();
        }

        if msg_len > self.back_buf.available_space() {
            println!("the message seem to be longer than the buffer's available space");
            if msg_len - self.back_buf.available_space() + self.back_buf.capacity()
                > self.max_buffer_size
            {
                println!("error: message is too large to write to back buffer. Consider increasing proxy channel buffer size, current value is {}", self.back_buf.capacity());
                return false;
            }

            let new_len = msg_len - self.back_buf.available_space() + self.back_buf.capacity();
            self.back_buf.grow(new_len);
        }
        println!("writing on the back buffer...");

        if let Err(e) = self.back_buf.write(message) {
            println!("error: channel could not write to back buffer: {:?}", e);
            return false;
        }

        println!("writing a zero byte on the back buffer...");
        if let Err(e) = self.back_buf.write(&b"\0"[..]) {
            println!("error: channel could not write to back buffer: {:?}", e);
            return false;
        }

        println!("setting channel interest to writable");
        self.interest.insert(Ready::writable());

        true
    }

    pub fn write_message_blocking(&mut self, message: &Tx) -> bool {
        println!("converting to bytes");

        let message = &serde_json::to_string(message)
            .map(|s| s.into_bytes())
            .unwrap_or_else(|_| Vec::new());

        let msg_len = message.len() + 1;
        if msg_len > self.back_buf.available_space() {
            println!("shifting the back buffer");

            self.back_buf.shift();
        }

        if msg_len > self.back_buf.available_space() {
            println!("the message seem to be longer than the buffer's available space");

            if msg_len - self.back_buf.available_space() + self.back_buf.capacity()
                > self.max_buffer_size
            {
                println!("error: message is too large to write to back buffer. Consider increasing proxy channel buffer size, current value is {}", self.back_buf.capacity());
                return false;
            }

            let new_len = msg_len - self.back_buf.available_space() + self.back_buf.capacity();
            self.back_buf.grow(new_len);
        }
        println!("writing on the back buffer...");

        if let Err(e) = self.back_buf.write(message) {
            println!("error: channel could not write to back buffer: {:?}", e);
            return false;
        }
        println!("writing a zero byte on the back buffer...");

        if let Err(e) = self.back_buf.write(&b"\0"[..]) {
            println!("error: channel could not write to back buffer: {:?}", e);
            return false;
        }

        loop {
            println!("Assessing the available data on the buffer...");
            let size = self.back_buf.available_data();
            if size == 0 {
                println!("no available space, we're done here");
                break;
            }

            println!("writing the buffer data onto the socket");
            match self.sock.write(self.back_buf.data()) {
                Ok(0) => {
                    println!("No bytes were written :-(");
                    return false;
                }
                Ok(r) => {
                    println!("Wrote {} bytes", r);
                    self.back_buf.consume(r);
                }
                Err(error) => {
                    println!("write error: {}", error);
                    // wait wait wait, shouldn't this return fals?
                    return true;
                }
            }
        }
        true
    }
}

impl<Tx: Debug + DeserializeOwned + Serialize, Rx: Debug + DeserializeOwned + Serialize>
    Channel<Tx, Rx>
{
    pub fn generate(
        buffer_size: usize,
        max_buffer_size: usize,
    ) -> io::Result<(Channel<Tx, Rx>, Channel<Rx, Tx>)> {
        let (command, proxy) = UnixStream::pair()?;
        let proxy_channel = Channel::new(proxy, buffer_size, max_buffer_size);
        let mut command_channel = Channel::new(command, buffer_size, max_buffer_size);
        command_channel.set_nonblocking(false);
        Ok((command_channel, proxy_channel))
    }

    pub fn generate_nonblocking(
        buffer_size: usize,
        max_buffer_size: usize,
    ) -> io::Result<(Channel<Tx, Rx>, Channel<Rx, Tx>)> {
        let (command, proxy) = UnixStream::pair()?;
        let proxy_channel = Channel::new(proxy, buffer_size, max_buffer_size);
        let command_channel = Channel::new(command, buffer_size, max_buffer_size);
        Ok((command_channel, proxy_channel))
    }
}

impl<Tx: Debug + Serialize, Rx: Debug + DeserializeOwned> Iterator for Channel<Tx, Rx> {
    type Item = Rx;
    fn next(&mut self) -> Option<Self::Item> {
        self.read_message()
    }
}

use mio::{Interest, Registry, Token};
impl<Tx, Rx> Source for Channel<Tx, Rx> {
    fn register(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        self.sock.register(registry, token, interests)
    }

    fn reregister(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        self.sock.reregister(registry, token, interests)
    }

    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        self.sock.deregister(registry)
    }
}
