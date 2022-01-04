// create a UnixStream that connects to the unix_listener of the receiver (=the server)
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
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

    let unix_stream =
        UnixStream::connect(socket_path).context("Could not connect to unix socket")?;

    write_request_onto_stream(unix_stream)
        .context("Could not write request onto the unix stream")?;

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

/*
fn write_ten_processing_and_one_error_on_stream(mut stream: UnixStream) -> anyhow::Result<()> {
    for i in 0..10usize {
        let processing_response = CommandResponse::new(
            format!("processing-{}", i),
            CommandStatus::Processing,
            "Not done yet".to_string(),
        );

        stream
            .write_all(format!("{:?}", processing_response))
            .context("Could not write this processing message on the unix stream")?;

        sleep(Duration::from_secs(1));
    }

    let error_response = CommandResponse::new(
        format!("error response"),
        CommandStatus::Error,
        "I am done and I have failed".to_string(),
    );

    println!("Sending response: {:?}", error_response);

    stream
        .write_all(format!("{:?}", error_response))
        .context("Could not write this error message on the unix stream")?;

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
