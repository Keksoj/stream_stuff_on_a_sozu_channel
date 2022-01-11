// create a channel that talks with the server
use std::{thread::sleep, time::Duration};

// use anyhow::{bail, Context};

use stream_stuff_on_a_sozu_channel::{
    channels::create_client_channel,
    command::{CommandRequest, CommandResponse, CommandStatus},
    copy_pasted_from_sozu::channel::Channel,
};

fn main() -> anyhow::Result<()> {
    let socket_path = "socket";

    // the socket should have been create prior to this, on the server side
    let mut sending_channel = create_client_channel(socket_path, true)?;

    // create a request
    let request = CommandRequest::new("My-urgent-request".to_string(), None);

    // send the request with the channel
    println!("Sending request: {:?}", request);
    match sending_channel.write_message(&request) {
        true => println!("Sending successful"),
        false => println!("Sending failed"),
    };

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
