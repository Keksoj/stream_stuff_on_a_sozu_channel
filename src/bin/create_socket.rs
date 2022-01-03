// Let's find out what this file descriptor thing is about
use stream_stuff_on_a_sozu_channel::socket::SocketBuilder;

fn main() -> anyhow::Result<()> {
    let path = "socket";

    let _socket = SocketBuilder::new()
        .with_path(path)?
        .with_permissions(0o700)?
        .build()?;

    Ok(())
}
