use stream_stuff_on_a_sozu_channel::copy_pasted_from_sozu::ready::Ready;

fn main() {
    let empty = Ready::empty();
    println!("empty: {:b}", empty);

    let mut readable = Ready::readable();
    println!("readable: {:b}", readable.0);

    readable.insert(Ready::writable());
    println!("readable+writable: {:b}", readable);
}
