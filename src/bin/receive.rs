use stream_stuff_on_a_sozu_channel::{
    channels::{create_receiving_channel, create_sending_channel},
    command::{CommandRequest, CommandResponse, CommandStatus},
    copy_pasted_from_sozu::channel::Channel,
    socket::{Socket, SocketBuilder},
};

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let socket_path = "socket";

    let _socket = SocketBuilder::new()
        .with_path(socket_path)?
        .with_permissions(777)?
        .build()?;

    let receiving_channel = create_receiving_channel(socket_path)?;

    Ok(())
}

fn receive_messages(mut channel: Channel<CommandRequest, CommandResponse>) {
    println!("Listening…");
    while let Some(response) = channel.read_message() {
        println!("Received response: {:?}", response);
    }
    println!("received_nothing…");
}
