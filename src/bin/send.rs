use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    thread::sleep,
    time::Duration,
};

use stream_stuff_on_a_sozu_channel::{
    channels::{create_receiving_channel, create_sending_channel},
    command::{CommandRequest, CommandResponse, CommandStatus},
    copy_pasted_from_sozu::channel::Channel,
    socket::{Socket, SocketBuilder},
};

fn main() -> anyhow::Result<()> {
    let socket_path = "socket";

    let mut sending_channel = create_sending_channel(socket_path)?;

    send_ten_processing_responses_and_then_error(sending_channel);

    Ok(())
}

fn send_ten_processing_responses_and_then_error(
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
