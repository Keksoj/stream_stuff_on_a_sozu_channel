mod command;
mod copy_pasted_from_sozu;

use anyhow::{bail, Context};
use command::{CommandRequest, CommandResponse};
use copy_pasted_from_sozu::channel::Channel;

fn main() {
    println!("Hello, world!");
}

pub fn create_channel(
    command_socket_path: &str,
) -> anyhow::Result<Channel<CommandRequest, CommandResponse>> {
    let mut channel = Channel::from_path(
        command_socket_path,
        16384,  // Sōzu default config
        163840, // Sōzu default config
    )
    .with_context(|| "Could not create Channel from the given path")?;

    channel.set_nonblocking(false);
    Ok(channel)
}
