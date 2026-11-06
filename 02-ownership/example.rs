// Experiment 02: Copy, move, shared borrow, mutable borrow.
// Compile: rustc --edition=2021 -g -C opt-level=0 example.rs -o example

#[derive(Debug)]
struct CanMessage {
    id: u32,
    payload: Vec<u8>,
}

fn inspect(message: &CanMessage) {
    println!("inspect: id={:#x}, len={}", message.id, message.payload.len());
}

fn change_first_byte(message: &mut CanMessage) {
    message.payload[0] = 0xAA;
}

fn consume(message: CanMessage) {
    println!("consume: {:?}", message);
} // message and its Vec are dropped here.

fn main() {
    // A. Copy: u32 implements Copy, so both variables remain usable.
    let can_id: u32 = 0x123;
    let copied_id = can_id;
    println!("copy: {can_id:#x}, {copied_id:#x}");

    // B. Move: Vec<u8> is not Copy. Ownership moves to 'message'.
    let payload = vec![0x02, 0x01, 0x0C];
    let mut message = CanMessage { id: can_id, payload };
    // println!("{payload:?}"); // ERROR: value borrowed after move
    println!("owned: {message:?}"); // breakpoint B

    // C. Shared borrowing: inspect does not take ownership.
    inspect(&message);
    inspect(&message);
    println!("after shared borrows: {message:?}"); // breakpoint C

    // D. Mutable borrowing: change the original; no ownership transfer.
    change_first_byte(&mut message);
    println!("after mutable borrow: {message:?}"); // breakpoint D

    // E. Borrow scopes: immutable borrow must not be used during a mutation.
    let first = &message.payload[0];
    println!("first byte before mutation: {first:#x}");
    message.payload[0] = 0xBB; // allowed because 'first' is no longer used
    println!("after mutation: {message:?}"); // breakpoint E

    // F. Move into a function; message cannot be used afterwards.
    consume(message);
    // println!("{message:?}"); // ERROR: use of moved value
}
