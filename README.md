# RUST
Rust is a powerful, performance-oriented, and memory-safe programming language. It’s known for its robust type system and fearless concurrency.
- Memory Safety Without Garbage Collection: Rust ensures memory safety through ownership and borrowing rules.
- Zero-Cost Abstractions: High-level abstractions don’t come with runtime overhead.
- Concurrency Without Fear: Rust prevents data races at compile time.

## Rustup
Recommended tool for managing rust versions & associated tools

## Ownership, Borrowing & Lifetimes
- **Ownership**: Each value has a single owner, and when the owner goes out of scope, the value is dropped.
- **Borrowing**: References to values can be borrowed without transferring ownership.
- **Lifetimes**: Ensure references are valid for their intended scope.

## Common Rust Features
- structs
- enums & pattern matching
- traits (Rust's version of interfaces)

## Memory Management
Rust doesn’t use a garbage collector. You'll need to:
- Use smart pointers like _Box, Rc, and RefCell_.
- Learn about stack vs. heap allocation.

## Error Handling
_Result_ and _Option_ enums are used instead of exceptions.

## Cargo & Crates
- _cargo_ - Rust's package manager & build system
- _crates_ - Rust's term for packages

## Tools
- Clippy: Linter for catching common mistakes.
    > cargo install clippy
    > cargo clippy
- Rustfmt: Formatter for keeping your code tidy.
    > cargo fmt
- Documentation: Automatically generate docs
    > cargo doc

![Rust vs Java/Python vs C++](image.png)

## Types of projects - Binaries & Libraries
![Binary vs Library](image-1.png)