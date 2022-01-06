# Stream stuff on a Sōzu channel

A number of [Sōzu](https://github.com/sozu-proxy/sozu) issues I work on have to deal with the CLI, and it's mostly a timeout thing.

After [removing the existing macros](https://github.com/sozu-proxy/sozu/pull/724) the timeouts still fail miserably.
I see myself forced to dig into the inner workings of the channel Sōzu works on.
The Channel is based on [mio](https://crates.io/crates/mio), a low-level asynchronous I/O library.

## What I work on

I copy-pasted those files that define the working of a Sōzu channel

-   `/command/src/channel.rs`
-   `/command/src/ready.rs`
-   `/command/src/buffer/mod.rs`
-   `/command/src/buffer/growable.rs`
-   `/command/src/buffer/fixed.rs`

The channel looks like this:

```rust
pub struct Channel<Tx, Rx> {
    pub sock:               mio::net::UnixStream,
        front_buf:          crate::buffer::growable::Buffer,
    pub back_buf:           crate::buffer::growable::Buffer,
        max_buffer_size:    usize,
    pub readiness:          crate::ready::Ready,
    pub interest:           crate::ready::Ready,
        blocking:           bool,
        phantom_tx:         std::marker::PhantomData<Tx>,
        phantom_rx:         std::marker::PhantomData<Rx>,
}

// /src/buffer/growable.rs
pub struct Buffer {
    memory: Vec<u8>,
    capacity: usize,
    position: usize,
    end: usize,
}

// /src/ready.rs
pub struct Ready(pub u16);

const READABLE: u16 = 0b00001;
const WRITABLE: u16 = 0b00010;
const ERROR: u16 = 0b00100;
const HUP: u16 = 0b01000;
```

## The goal

What I ultimately want to do is to read several messages until the business logic is done.
In pseudo-rust:

```rust
while Some(message) = channel.read_message_with_timeout(Duration::from_secs(2)) {
    match message.status {
        Status::Error => break,
        Status::Processing => {
            println!("processing: {}", message.message);
            // wait for the next message
        },
        Status::Ok => {
            println!("Success: {}", message.message);
            break;
        }
    }
}
```

To implement the timeout, I need non-blocking channels.

## What is a file descriptor ?

    cargo run --bin rawfd

It turns out if we want the raw file descriptor of a file we have to use `std::os::unix::io::AsRawFd`.

## Make a socket-based client/server

In order to understand how unix sockets work and what a unix stream is, I wrote:

- a small socket library to create a socket, `src/socket.rs`
- a simple socket-based server that creates a socket, uses its listener, writes responses back
- a simple socket-based client that connects to the socket, writes to the stream and read from it.

In two separate terminals, run first:

    cargo run --bin socket_server

and then:

    cargo run --bin socket_client

