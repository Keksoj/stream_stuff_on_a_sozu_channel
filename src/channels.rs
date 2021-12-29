use crate::command::{CommandRequest, CommandResponse, CommandStatus};
use crate::copy_pasted_from_sozu::channel::Channel;

use anyhow::{bail, Context};
use std::thread::sleep;
use std::time::Duration;

pub fn create_receiving_channel(
    command_socket_path: &str,
) -> anyhow::Result<Channel<CommandRequest, CommandResponse>> {
    let mut channel = Channel::from_path(
        command_socket_path,
        16384,  // default Sōzu config
        163840, // default Sōzu config
    )
    .with_context(|| "Could not create Channel from the given path")?;

    channel.set_nonblocking(false);
    Ok(channel)
}

pub fn create_sending_channel(
    command_socket_path: &str,
) -> anyhow::Result<Channel<CommandResponse, CommandRequest>> {
    let mut channel = Channel::from_path(
        command_socket_path,
        16384,  // default Sōzu config
        163840, // default Sōzu config
    )
    .with_context(|| "Could not create Channel from the given path")?;

    channel.set_nonblocking(false);
    Ok(channel)
}

pub fn send_ten_processing_responses_and_then_error(
    mut channel: Channel<CommandResponse, CommandRequest>,
) {
    for i in 0..10usize {
        let processing_response = CommandResponse::new(
            format!("processing-{}", i),
            CommandStatus::Processing,
            "Not done yet".to_string(),
        );

        println!("Sending response: {:?}", processing_response);
        channel.write_message(&processing_response);

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
