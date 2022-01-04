## Why do I get this error every time?

Even when setting socket permissions to `777`, I can't connect to it:

```
Error: Can not connect to socket

Caused by:
    Permission denied (os error 13)
```

It turns out the permissions had to be given in octal, so `0o600` (600 is plenty).

# Work it the verbose way first

Instead of taming the channels head-on, let's try and get it to work using only sockets.

## Create a listener

Listening on a socket works when doing this on the server side:

```rust
fn main() -> anyhow::Result<()> {
    let socket = SocketBuilder::new()
        .with_path(socket_path)?
        .with_permissions(0o700)?
        .build()?;

    for unix_stream in socket.listener.incoming() {
        match unix_stream {
            Ok(stream) => handle_stream(stream)?,
            Err(e) => {
                bail!(format!("{}", e));
            }
        }
    }
    Ok(())
}

fn handle_stream(mut stream: UnixStream) -> anyhow::Result<()> {
    println!("stream: {:?}", stream);
    let mut message = String::new();
    stream.read_to_string(&mut message)?;
    println!("{}", message);
    Ok(())
}
```

But this is blocking. If the client sends ten messages on the socket, the `read_to_string` function waits for them all.

## How to make the listener non-blocking?

I tried copying Sōzu's code but its asynchronous wrapping sounds like chinese to me.

Let's using the `set_nonblocking` method of `std::os::unix::net::UnixListener`.
Here is the documentation:

> This will result in the `accept` operation becoming nonblocking,
> i.e., immediately returning from their calls. If the IO operation is
> successful, `Ok` is returned and no further action is required. If the
> IO operation could not be completed and needs to be retried, an error
> with kind [`io::ErrorKind::WouldBlock`] is returned.

So I gave the SocketBuilder a nice `nonblocking(boolean)` method and… I've got mixed feelings about this. Calling the `accept()` method on a non-blocking listener yielded this error:

    Error: Failed at accepting a connection on the unix listener
    Caused by:
        Resource temporarily unavailable (os error 11)

That being said, I managed to make it all work with a blocking socket.

**Server side**:

```rust
// within main
    loop {
        let (unix_stream, socket_address) = socket
            .listener
            .accept()
            .context("Failed at accepting a connection on the unix listener")?;
        handle_stream(unix_stream)?;
    }

fn handle_stream(mut stream: UnixStream) -> anyhow::Result<()> {
    let mut message = String::new();
    stream
        .read_to_string(&mut message)
        .context("Failed at reading the unix stream")?;

    Ok(())
}
```

**client side**:

```rust
// within main

    let unix_stream =
        UnixStream::connect(socket_path).context("Could not connect to unix socket")?;

    stream.write(request_as_bytes)?;
//
```

This prints the request on the server.
