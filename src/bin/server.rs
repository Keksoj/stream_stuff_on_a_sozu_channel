// this is a server
// create a unix_listener that accepts connections from client
use std::{thread::sleep, time::Duration};

use anyhow::{bail, Context};
use mio::net::UnixStream;

use stream_stuff_on_a_sozu_channel::{
    channels::create_server_channel,
    command::{CommandRequest, CommandResponse, CommandStatus},
    copy_pasted_from_sozu::channel::Channel,
    socket::SocketBuilder,
};
/*
    channels::{create_receiving_channel, create_sending_channel},
*/

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let socket_path = "socket";

    // create the socket
    let socket = SocketBuilder::new()
        .with_path(socket_path)
        .with_permissions(0o700)
        .build()
        .context("Could not create the socket")?;

    // the loop allows to handle several connections, one after the other
    loop {
        // accept_connection() is a wrapper around UnixListener::accept(), check socket.rs
        let (unix_stream, socket_address) = socket
            .accept_connection()
            .context("Can not create unix stream")?;

        println!(
            "Accepted connection. Stream: {:?}, address: {:?}",
            unix_stream, socket_address
        );

        // handle_connection(unix_stream)?;
    }
    Ok(())
}
/*
fn handle_connection(mut unix_stream: UnixStream) -> anyhow::Result<()> {
    let mut channel = create_server_channel(unix_stream, true);

    // receive a message using normal read logic
    let mut message = String::new();
    channel
        .read_to_string(&mut message)
        .context("Failed at reading the unix stream")?;

    println!("{}", message);

    // parse it, it should be a JSON request
    let request = serde_json::from_str::<Request>(&message)
        .context("could no deserialize request message")?;

    println!("Parsed this request: {:?}", request);

    // Emulate processing time
    // send 10 processings responses every second before sending the final one
    for _ in 0..9 {
        let mut processing = CommandResponse::new(
            "processing",
            CommandStatus::Processing,
            "still processing...",
        )
        .serialize_to_bytes()
        .context("Could not serialize response")?;

        // add a zero byte, to separate instructions
        processing.push(0);

        // pretty normal write logic
        println!("Sending processing response");
        channel
            .write_message(&processing)
            .context("Could not write processing response onto the unix stream")?;

        sleep(Duration::from_secs(1));
    }

    // create a response that matches the request
    let response: CommandResponse = match request.id.as_str() {
        "request" => CommandResponse::new("response", CommandStatus::Ok, "Roger that"),
        _ => CommandResponse::new("what", CommandStatus::Error, "Sorry what?"),
    };

    let mut response_as_bytes = response
        .serialize_to_bytes()
        .context("Could not serialize response")?;

    // the zero byte is a separator, so that the client can distinguish between responses
    response_as_bytes.push(0);

    // the usual write logic
    channel
        .write_message(&response_as_bytes)
        .context("Could not write response onto the unix stream")?;

    Ok(())
}

fn send_ten_processing_and_one_error_with_channel(
    mut channel: Channel<CommandResponse, CommandRequest>,
) {
    for i in 0..10usize {
        let processing_response = CommandResponse::new(
            format!("processing-{}", i),
            CommandStatus::Processing,
            "Not done yet".to_string(),
        );

        if channel.write_message(&processing_response) {
            println!("Sending response: {:?}", processing_response);
        } else {
            println!("Could not send: {:?}", processing_response);
        }

        sleep(Duration::from_secs(1));
    }

    let error_response = CommandResponse::new(
        format!("error response"),
        CommandStatus::Error,
        "I am done and I have failed".to_string(),
    );

    println!("Sending response: {:?}", error_response);
    channel.write_message(&error_response);
}
*/
