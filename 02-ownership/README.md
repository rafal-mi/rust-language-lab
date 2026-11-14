# 02 — Ownership, moves, Copy, and borrowing

This is a standalone Rust-language experiment, inspired by the automotive CAN simulator. No Cargo or crates are needed.

## Build and run (Linux)

Open **this directory** in a terminal or VS Code.

```sh
rustc --edition=2021 -g -C opt-level=0 example.rs -o example
./example
```

The source labels six checkpoints A–F. Pause there and predict **which variables remain usable**, before running the program.

## Concepts

1. `Copy`: assigning a `u32` copies its value; the source variable remains usable. `Copy` is a trait, not a property of every stack value.
2. **Move**: moving a `Vec<u8>` into a struct transfers ownership. Rust prevents using the former owner. The move does not necessarily imply copying its heap allocation.
3. `&T`: a shared borrow gives read-only access through that reference. It does not transfer ownership.
4. `&mut T`: an exclusive mutable borrow permits mutation while the borrowing rules are satisfied.
5. **Non-lexical lifetimes**: in checkpoint E, `first` is no longer used after its print, so a later mutation is allowed even though the binding remains in lexical scope.
6. Passing `CanMessage` **by value** to `consume` moves ownership. Its destructor runs when that owned value goes out of scope.

**Compiler guarantee vs. programmer responsibility:** safe Rust prevents use-after-move, dangling references, and conflicting borrows. It does not verify that a CAN ID or payload is valid for your protocol.

## Debug with GDB

```sh
gdb ./example
```

In GDB:

```gdb
break main
run
list
next
info locals
print message
print message.payload
continue
quit
```

Better yet, place a breakpoint at a source line marked `breakpoint B`, C, D, or E using `break example.rs:<line>` (get the line number from `nl -ba example.rs`).

Compare the payload before and after `change_first_byte`. Some debug views of Rust's standard library types vary with GDB/rustc versions. **Do not infer ownership from raw bytes alone**: a move is primarily a compile-time restriction on which variable may be used; its generated machine operations can vary.

## Debug with VS Code

Open **02-ownership as the workspace folder** and press F5 using the included CodeLLDB configuration. The pre-launch task invokes `rustc`, not Cargo. Set breakpoints on the checkpoint comments and inspect `message` in Variables. Use F10 to step over statements.

## Explore compiler errors

```sh
rustc --edition=2021 compile_fail.rs
```

This file is intentionally invalid: study errors **E0382** (use after move) and **E0502** (conflicting borrows). Do not include it in the normal build task.

Try commenting out one invalid section at a time. Then change the `String` example to `u32` and explain why it compiles.

## Questions

- Why can both `can_id` and `copied_id` be printed?
- Why is `payload` unavailable after it is moved into `CanMessage`?
- Why can `inspect(&message)` be called twice?
- What would happen if `println!("{first}")` were moved **after** `message.payload[0] = 0xBB`?
- Does `consume(message)` necessarily clone or byte-copy the vector allocation? (No.)

Note: manipulating a vector in a debugger and looking at its data is useful; the borrow checker does not run at debug time.
