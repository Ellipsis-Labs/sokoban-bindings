# `sokoban-bindings`: using `sokoban` in C/C++/Zig

Rust offers a powerful type system with generics, similar to C++ templates. However, structs and functions with generics cannot be used in C via FFI without monomorphization (i.e. picking a single instance of the generic type or function). This crate defines a macro for the red black tree in `sokoban` that facilitates such monomorphization, exposing a simple C-compatible interface for tree initialization, insertion, removal, and retrieval.

This helper macro is defined in `sokoban-bindings`, and an example of its use is provided in `sokoban-bindings-example`.

# Usage
The `sokoban-bindgen` crate defines the monomorphization helper macro. You must use this macro in your own rust library via
```rust
red_black_tree_bindings!(key_type, value_type, capacity);
```
This will define C-compatible types and functions that use the  monomorphized `sokoban` types and methods. 

You will need 
- `lib-sokoban`
- The `cbindgen` crate to generate the C headers, with nightly since the expand feature requres it. (The `sokoban-bindgen` macro must be expanded before cbindgen can parse and generate headers)
- The `concat-idents` crate. This is re-exported within `sokoban-bindgen`, so you can get it via `use sokoban_bindgen::*`.

# The `sokoban-bindings-example` example
This crate shows how to generate the staticlib and use the bindgen macro and cbindgen to generate a C header for sokoban. In particular, you should note:
1. The `build.rs` within the crate
2. The `staticlib` option specified within the `[lib]` section of the Cargo.toml
3. The use of `use sokoban_bindgen::*`, which imports the `concat_idents` macro with a particular name


To use the example in C or Zig:
1. To build the static library and header via cargo, use `cargo +nightly build --release -p sokoban-bindings-example`. Nightly is required for macro expansion. The header will be written to `examples/c/sokoban.h` .

**To build and run the C example**: The makefile provided will do the same thing as step 1 when `make` is invoked, in addition to building the C example. The executable will be written to `examples/c/sokoban.out`.

**To run the Zig Example**: After building the static library and header via step 1 or via `make`, navigate to examples/zig and `zig build run`.


# Notes
1. `Option<T>` is used by sokoban to communicate insertion and retrieval success and failure, but it is not FFI safe. Two values `SUCCESS = 0` and `FAILURE = u32::MAX` were introduced to deal with this[^1]. In lieu of an `Option`, these values communicates to the C or Zig application that an attempt to insert or retrieve a value failed or succeeded. As an example, for the red black tree:

    1. Instead of `insert` returning an `Option<u32>` node address which is `None` when insertion fails, the function returns `FAILURE` if it failed to insert or the node address (also a `u32`) if it succeeded.
    2. Instead of `get` returning an `Option<&V>`, or `remove` returning an `Option<V>`, the function accepts a pointer to which the value will be copied if successful, along with `SUCCESS = 0`. If the retrieval fails, `FAILURE = u32::MAX` is returned and the pointer remains untouched (and perhaps uninitialized!).



[^1]: This limits the capacity of the node allocator to one less than its max value.

