// Intentionally DOES NOT COMPILE. Use rustc to read the diagnostics.
// Compile: rustc --edition=2021 compile_fail.rs
fn main() {
    let message = String::from("CAN");
    let moved = message;
    println!("{message}"); // E0382: borrow of moved value
    println!("{moved}");

    let mut bytes = vec![1u8, 2, 3];
    let first = &bytes[0];
    bytes.push(4); // E0502: mutable borrow while shared borrow remains in use
    println!("{first}");
}
