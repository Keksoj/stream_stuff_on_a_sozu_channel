use mio::net::{SocketAddr, UnixListener, UnixStream};
use std::io::Read;

use anyhow::{bail, Context};

fn main() -> anyhow::Result<()> {
    let socket_path = "socket";

    let unix_listener = std::os::unix::net::UnixListener::bind(socket_path).context("Could not create listener")?;

    loop {
        let (unix_stream, socket_address) = unix_listener
            .accept()
            .context("could not accept connection")?;

        println!(
            "Accepted connection. Stream: {:?}, address: {:?}",
            unix_stream, socket_address
        );

        handle_connection(unix_stream);
    }
    Ok(())
}

fn handle_connection(mut unix_stream: std::os::unix::net::UnixStream) -> anyhow::Result<()> {
    // receive a message using normal read logic
    let mut message = String::new();
    unix_stream
        .read_to_string(&mut message)
        .context("Failed at reading the unix stream")?;

    println!("{}", message);
    Ok(())
}
