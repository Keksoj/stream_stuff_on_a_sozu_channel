// this is a server
// create a unix_listener that accepts connections from client
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    thread::sleep,
    time::Duration,
};

use anyhow::{bail, Context};
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

    let mut socket = SocketBuilder::new()
        .with_path(socket_path)
        .with_permissions(0o700)
        .nonblocking(false)
        .build()
        .context("Could not create the socket")?;

    /*
    for unix_stream in socket.listener.incoming() {
        match unix_stream {
            Ok(stream) => handle_stream(stream).context("Failed at handling the unix stream")?,
            Err(e) => {
                bail!(format!(
                    "Error with incoming connection on the unix socket:\n    {}",
                    e
                ));
            }
        }
    }
    */

    loop {
        let (unix_stream, socket_address) = socket
            .listener
            .accept()
            .context("Failed at accepting a connection on the unix listener")?;
        println!("Accepted connection. Stream: {:?}, address: {:?}", unix_stream, socket_address);
        handle_stream(unix_stream)?;
    }

    Ok(())
}

fn handle_stream(mut stream: UnixStream) -> anyhow::Result<()> {
    
    let mut message = String::new();
    stream
        .read_to_string(&mut message)
        .context("Failed at reading the unix stream")?;

    println!("{}", message);
    Ok(())
}

