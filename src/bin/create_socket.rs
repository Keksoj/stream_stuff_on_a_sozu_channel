// Let's find out what this file descriptor thing is about
use std::{
    fs::File,
    io::{Read, Write},
    os::unix::{io::AsRawFd, net::UnixListener},
    path::Path,
};

use anyhow::Context;
use mio::net::UnixStream;

use stream_stuff_on_a_sozu_channel::socket::{Socket, SocketBuilder};

fn main() -> anyhow::Result<()> {
    let path = "./socket";

    let socket = SocketBuilder::new()
        .with_path(path)?
        .with_permissions(0o600)?
        .build()?;

    let mut stream = UnixStream::connect(path).context("Can not connect to socket")?;

    stream.write_all(b"hello world")?;

    let mut response = String::new();

    // stream.set_read_timeout(Some(std::time::Duration::from_secs(2)));
    stream
        .read_to_string(&mut response)
        .context("No message read")?;

    println!("{}", response);

    Ok(())
}


