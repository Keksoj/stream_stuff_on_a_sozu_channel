use mio::net::UnixStream;

use crate::{
    command::{CommandRequest, CommandResponse},
    copy_pasted_from_sozu::channel::Channel,
};

use anyhow::Context;

pub fn create_client_channel(
    command_socket_path: &str,
    blocking: bool,
) -> anyhow::Result<Channel<CommandRequest, CommandResponse>> {
    println!("Creating channel on socket `{}`", command_socket_path);
    let mut channel = Channel::from_path(
        command_socket_path,
        16384,  // default Sōzu config
        163840, // default Sōzu config
    )
    .with_context(|| "Could not create Channel from the given path")?;

    channel.set_nonblocking(!blocking);
    Ok(channel)
}

pub fn create_server_channel(
    unix_stream: UnixStream,
    blocking: bool,
) -> Channel<CommandResponse, CommandRequest> {
    println!("Creating channel on socket {:?}", unix_stream);

    let mut channel = Channel::new(
        unix_stream,
        16384,  // default Sōzu config
        163840, // default Sōzu config
    );

    channel.set_nonblocking(!blocking);
    channel
}
