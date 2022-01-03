// create, read and write on a std::os::unix::net::UnixStream instead of mio
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

    // my custom socket builder
    let socket = SocketBuilder::new()
        .with_path(path)?
        .with_permissions(0o700)?
        .build()?;

    let mut stream = UnixStream::connect(path).context("Can not connect to socket")?;

    stream.write_all(b"hello world")?;

    let mut response = String::new();

    stream.set_read_timeout(Some(std::time::Duration::from_secs(2)));
    stream
        .read_to_string(&mut response)
        .context("No message read")?;

    println!("{}", response);

    Ok(())
}
