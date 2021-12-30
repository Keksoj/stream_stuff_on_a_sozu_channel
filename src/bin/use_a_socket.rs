// Let's find out what this file descriptor thing is about
use std::{
    fs::File,
    io::{Read, Write},
    os::unix::{
        io::AsRawFd,
        net::{UnixListener, UnixStream},
    },
    path::Path,
};

use anyhow::Context;

use stream_stuff_on_a_sozu_channel::socket::{Socket, SocketBuilder};

fn main() -> anyhow::Result<()> {
    let path = "./socket";

    let socket = SocketBuilder::new()
        .with_path(path)?
        .with_permissions(777)?
        .build()?;

    let mut stream = UnixStream::connect(path).context("Can not connect to socket")?;

    stream.write_all(b"hello world")?;

    let mut response = String::new();

    stream.read_to_string(&mut response)?;

    println!("{}", response);

    Ok(())
}

fn create_socket_and_find_raw_fd(socket_path: &str) -> anyhow::Result<()> {
    println!("Let's create a socket at path {:?}", socket_path);

    let unix_socket = UnixListener::bind(socket_path).context("Could not create unix socket")?;

    let raw_fd = unix_socket.as_raw_fd();

    println!("Here is its raw file descriptor: {:?}", raw_fd);

    Ok(())
}
