use anyhow::Context;
use std::fs;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;

pub fn create_socket(command_socket_path: &str) -> anyhow::Result<()> {
    let address = PathBuf::from(command_socket_path);

    println!("{:?}", address);

    println!("Checking for the presence of a unix socket");
    if fs::metadata(&address).is_ok() {
        println!("There is one already!");
    } else {
        println!("No socket is present. Creating...");
        let unix_socket = UnixListener::bind(&address).context("Could not create unix socket")?;
        println!("Created socket {:?}", unix_socket)
    }

    Ok(())
}
