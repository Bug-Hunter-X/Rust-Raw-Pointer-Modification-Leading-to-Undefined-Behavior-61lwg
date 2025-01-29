# Rust Raw Pointer Example: Undefined Behavior

This repository demonstrates a potential issue with using raw pointers in Rust. Modifying the vector through a raw pointer could cause undefined behavior due to the vector's internal management of memory and capacity.

The `bug.rs` file contains code demonstrating this issue, while `bugSolution.rs` shows a safer alternative using proper Rust methods.