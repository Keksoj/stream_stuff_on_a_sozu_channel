### Debugging channel creation

The channels should not be creating by doing this:

1. passing a socket path as argument to `Channel::from_path()`
2. the `from_path()` function creates a unix stream

What should be done:

1. accept a connection on a unix listener => get a unix stream
2. passing this unix stream as argument to `Channel::new()`

Thanks @Keruspe.

Well but this yields yet another problem:

```
Error: Can not create unix stream

Caused by:
    0: Could not accept connection on socket `socket`
    1: Resource temporarily unavailable (os error 11)
```

It turns out `mio::net::UnixListener` is non-blocking by default, whereas  `std::os::unix::net::UnixListener` is blocking by default.
Yet another thing to debug, yay.