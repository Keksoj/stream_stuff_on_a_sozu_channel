use std::{
    fs::{metadata, remove_file, set_permissions, Permissions},
    os::unix::{fs::PermissionsExt, net::UnixListener},
    path::PathBuf,
};

use anyhow::{bail, Context};

#[derive(Debug)]
pub struct Socket {
    pub path: String,
    pub listener: UnixListener,
    pub permissions: Option<Permissions>,
}

pub struct SocketBuilder {
    path: Option<String>,
    listener: Option<UnixListener>,
    permissions: Option<Permissions>,
}

impl SocketBuilder {
    pub fn new() -> Self {
        Self {
            path: None,
            listener: None,
            permissions: None,
        }
    }

    pub fn with_path<T>(self, path: T) -> anyhow::Result<Self>
    where
        T: ToString,
    {
        Ok(Self {
            path: Some(path.to_string()),
            listener: self.listener,
            permissions: self.permissions,
        })
    }

    /// ex: "0o600"
    pub fn with_permissions(self, permissions: u32) -> anyhow::Result<Self> {
        let permissions = Permissions::from_mode(permissions);

        println!("Permissions are set.");

        Ok(Self {
            path: self.path,
            listener: self.listener,
            permissions: Some(permissions),
        })
    }

    pub fn build(self) -> anyhow::Result<Socket> {
        println!("Creating socket...");
        if self.path.is_none() {
            bail!("Please provide a path first");
        }

        let cloned_path = self.path.clone().unwrap();
        let addr = self.path.unwrap();

        println!(
            "Checking for the presence of a unix socket at path `{}`",
            addr
        );

        if metadata(&addr).is_ok() {
            println!("A socket is already present. Deleting...");
            remove_file(&addr)
                .with_context(|| format!("could not delete previous socket at {:?}", addr))?;
        }

        let unix_listener = UnixListener::bind(&addr).context("could not create unix socket")?;

        if self.permissions.is_some() {
            set_permissions(&addr, self.permissions.clone().unwrap())
                .context("could not set the unix socket permissions.")?;
        } else {
            println!("Warning, no permissions set.")
        }

        let socket = Socket {
            path: cloned_path,
            listener: unix_listener,
            permissions: self.permissions,
        };

        println!("Successfully created socket: {:#?}", socket);

        Ok(socket)
    }
}
