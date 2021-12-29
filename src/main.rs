mod channels;
mod command;
mod copy_pasted_from_sozu;
mod socket;

use anyhow::{bail, Context};
use channels::{
    create_receiving_channel, create_sending_channel, send_ten_processing_responses_and_then_error,
};
use command::{CommandRequest, CommandResponse, CommandStatus};
use copy_pasted_from_sozu::channel::Channel;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let socket_path = "socket";

    let mut sending_channel = create_sending_channel(socket_path)?;

    let mut receiving_channel = create_receiving_channel(socket_path)?;

    let sender = std::thread::spawn(move || {
        send_ten_processing_responses_and_then_error(sending_channel);
    });

    let receiver = std::thread::spawn(move || {
        receive_messages(receiving_channel);
    });

    println!("hi!");
    sender.join().expect("the thread crashed");
    receiver.join().expect("the thread crashed");
    Ok(())
}

fn receive_messages(mut channel: Channel<CommandRequest, CommandResponse>) {
    while let Some(response) = channel.read_message() {
        println!("{:?}", response);
    }
}
