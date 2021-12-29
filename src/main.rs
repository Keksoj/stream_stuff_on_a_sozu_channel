mod lib;

use lib::channel::Channel;

fn main() {
    println!("Hello, world!");
}

/*
pub fn create_channel(command_socket_pacg: String) -> anyhow::Result<Channel<CommandRequest, CommandResponse>> {
    let mut channel = Channel::from_path(
        &config.command_socket_path()?,
        config.command_buffer_size,
        config.max_command_buffer_size,
    )
    .with_context(|| "Could not create Channel from the given path")?;

    channel.set_nonblocking(false);
    Ok(channel)
}
*/
