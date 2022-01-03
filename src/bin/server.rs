// this is a server
// create a unix_listener that accepts connections from client
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    thread::sleep,
    time::Duration,
};

use anyhow::bail;
use async_io::Async;

use stream_stuff_on_a_sozu_channel::socket::{Socket, SocketBuilder};
/*
    channels::{create_receiving_channel, create_sending_channel},
    command::{CommandRequest, CommandResponse, CommandStatus},
    copy_pasted_from_sozu::channel::Channel,
*/

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let socket_path = "socket";

    let socket = SocketBuilder::new()
        .with_path(socket_path)?
        .with_permissions(0o700)?
        .build()?;

    for unix_stream in socket.listener.incoming() {
        match unix_stream {
            Ok(stream) => handle_stream(stream)?,
            Err(e) => {
                bail!(format!("{}", e));
            }
        }
    }

    println!("done");

    Ok(())
}

fn handle_stream(mut stream: UnixStream) -> anyhow::Result<()> {
    println!("stream: {:?}", stream);
    let mut message = String::new();
    stream.read_to_string(&mut message)?;
    println!("{}", message);
    Ok(())
}
