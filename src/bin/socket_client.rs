// create a UnixStream that connects to the unix_listener of the receiver (=the server)
use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    thread::sleep,
    time::Duration,
};

use anyhow::{bail, Context};

use stream_stuff_on_a_sozu_channel::{
    channels::{create_receiving_channel, create_sending_channel},
    command::{CommandRequest, CommandResponse, CommandStatus},
    copy_pasted_from_sozu::channel::Channel,
    socket::{Socket, SocketBuilder},
};

fn main() -> anyhow::Result<()> {
    let socket_path = "socket";

    // let mut sending_channel = create_sending_channel(socket_path, false)?;

    // Connect to the socket
    let unix_stream =
        UnixStream::connect(socket_path).context("Could not connect to unix socket")?;

    write_request_onto_stream(unix_stream)
        .context("Could not write request onto the unix stream")?;

    // listen to responses
    let unix_listener = UnixListener::bind(socket_path).context("Could not bind to the socket")?;
    loop {
        let (unix_stream, socket_address) = unix_listener
            .accept()
            .context("Failed at accepting a connection on the unix listener")?;
        println!(
            "Accepted connection. Stream: {:?}, address: {:?}",
            unix_stream, socket_address
        );
        handle_response(unix_stream)?;
    }

    Ok(())
}

fn write_request_onto_stream(mut stream: UnixStream) -> anyhow::Result<()> {
    let request = CommandRequest::new("My-urgent-request".to_string(), None);

    let request_as_string = request
        .to_serialized_string()
        .context("failed at serializing request")?;

    let request_as_bytes = request_as_string.as_bytes();

    stream
        .write(request_as_bytes)
        .context("Writing bytes failed")?;
    println!("This request has been written : {:?}", request_as_string);

    Ok(())
}

fn handle_response(mut stream: UnixStream) -> anyhow::Result<()> {
    // receive a message
    let mut message = String::new();
    stream
        .read_to_string(&mut message)
        .context("Failed at reading the unix stream")?;

    println!("{}", message);

    // parse it, it is a request after all
    let response = serde_json::from_str::<CommandResponse>(&message)
        .context("could no deserialize request message")?;

    println!("Received this response: {:?}", response);

    Ok(())
}
