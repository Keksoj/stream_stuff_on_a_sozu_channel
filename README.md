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

## Client/server

The client sends a request, the server is _supposed_ to receive it. Run first:

    cargo run --bin server

You may listen to what happens on the created socket by using netcat in another terminal:

    netcat -lkU socket

and then, in a third terminal:

    cargo run --bin client

## Readiness

What is this `Ready` thing? What are those binary operators doing? Let's find out.

    cargo run --bin ready