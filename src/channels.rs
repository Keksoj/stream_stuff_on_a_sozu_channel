use crate::command::{CommandRequest, CommandResponse, CommandStatus};
use crate::copy_pasted_from_sozu::channel::Channel;

use anyhow::{bail, Context};

pub fn create_receiving_channel(
    command_socket_path: &str,
) -> anyhow::Result<Channel<CommandRequest, CommandResponse>> {
    println!("Creating channel on socket `{}`", command_socket_path);
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
    println!("Creating channel on socket `{}`", command_socket_path);

    let mut channel = Channel::from_path(
        command_socket_path,
        16384,  // default Sōzu config
        163840, // default Sōzu config
    )
    .with_context(|| "Could not create Channel from the given path")?;

    channel.set_nonblocking(false);
    Ok(channel)
}
