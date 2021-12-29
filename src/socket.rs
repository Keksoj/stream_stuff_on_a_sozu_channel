use anyhow::Context;
use std::fs;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;

pub fn create_socket(command_socket_path: &str) -> anyhow::Result<UnixListener> {
    let address = PathBuf::from(command_socket_path);

    // copied from Sōzu he he
    if fs::metadata(&address).is_ok() {
        println!("A socket is already present. Deleting...");
        fs::remove_file(&address)
            .with_context(|| format!("could not delete previous socket at {:?}", address))?;
    }

    let unix_socket = UnixListener::bind(&address).context("Could not create unix socket")?;

    Ok(unix_socket)
}
