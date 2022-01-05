// this is a server
// create a unix_listener that accepts connections from client
use std::{
    io::{Read, Write},
    thread::sleep,
    time::Duration,
};

use anyhow::{bail, Context};

use stream_stuff_on_a_sozu_channel::{
    channels::{create_client_channel, create_server_channel},
    command::{CommandRequest, CommandResponse, CommandStatus},
    copy_pasted_from_sozu::channel::Channel,
    socket::{Socket, SocketBuilder},
};
/*
    channels::{create_receiving_channel, create_sending_channel},
*/

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let socket_path = "socket";

    // create the socket
    let mut socket = SocketBuilder::new()
        .with_path(socket_path)
        .with_permissions(0o700)
        .nonblocking(false)
        .build()
        .context("Could not create the socket")?;

    // create the channel
    let mut receiving_channel = create_server_channel(socket_path, true)?;

    while let Some(response) = receiving_channel.read_message() {
        println!("{:?}", response);
    }

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
