use anyhow::Context;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;

pub fn create_socket(command_socket_path: &str) -> anyhow::Result<()> {
    let address = PathBuf::from(command_socket_path);

    let unix_listener = UnixListener::bind(&address).context("Could not create unix socket")?;

    
    Ok(())
}
