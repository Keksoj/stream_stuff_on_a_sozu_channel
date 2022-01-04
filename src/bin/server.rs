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

use stream_stuff_on_a_sozu_channel::{
    command::{CommandRequest, CommandResponse, CommandStatus},
    socket::{Socket, SocketBuilder},
};
/*
    channels::{create_receiving_channel, create_sending_channel},
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
        println!(
            "Accepted connection. Stream: {:?}, address: {:?}",
            unix_stream, socket_address
        );
        handle_stream(unix_stream)?;
    }

    Ok(())
}

fn handle_stream(mut stream: UnixStream) -> anyhow::Result<()> {
    // receive a message
    let mut message = String::new();
    stream
        .read_to_string(&mut message)
        .context("Failed at reading the unix stream")?;

    println!("{}", message);

    // parse it, it is a request after all
    let request = serde_json::from_str::<CommandRequest>(&message)
        .context("could no deserialize request message")?;

    println!("Parsed this request: {:?}", request);

    // send back a bunch of messages
    let response: CommandResponse = match request.id.as_str() {
        "reply" => CommandResponse::new("response", CommandStatus::Ok, "Roger that"),
        _ => CommandResponse::new("what", CommandStatus::Ok, "Sorry what?"),
    };

    let response_as_string = response
        .to_serialized_string()
        .context("Could not serialize response")?;

    let response_as_bytes = response_as_string.as_bytes();

    stream
        .write(response_as_bytes)
        .context("Could not write response onto the unix stream")?;

    Ok(())
}
