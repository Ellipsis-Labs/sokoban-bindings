# `sokoban-bindings`: using `sokoban` in C/C++/Zig

Rust offers a powerful type system with generics, similar to C++ templates. However, structs and functions with generics cannot be used in C via FFI without monomorphization (e.g. picking a single instance of the generic type or function). This crate defines a macro for the red black tree in `sokoban` that facilitates such monomorphization and exposes a simple interface for initialization, insertion, removal, and retrieval of types from the tree.

This helper macro is defined in `sokoban-bindings`, and an example of its use is provided in `sokoban-bindings-example`. In particular, you will also want to look at the cbindgen code in `build.rs`, and you will have to use nightly for macro expansion.

# Usage
To build the header via cargo, use `cargo +nightly build --release -p sokoban-bindings-example`. It will be output at `examples/c/sokoban.h` The makefile provided will do the same thing when `make` is invoked, in addition to building the C example.
