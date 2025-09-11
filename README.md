## Overview

This public repository contains the Rust source code for the algorithms and
clients in the text book <a href = "http://amzn.to/13VNJi7">Algorithms, 4th
Edition</a> by Robert Sedgewick and Kevin Wayne.

## Goals

My goal is to cover all the algorithms code that the [official algs4
repo](https://github.com/kevin-wayne/algs4 "official algs4 repo") provides,
implementing them in Rust, trying to keep the APIs compatible to the official
Java version.  In some cases, the APIs are designed to be idiomatic Rust, for
example, I/O APIs, `len()` instead of `size()`, and `Option<&T>` instead of
`NoSuchElementException`.

I hope that this crate helps people learn both Algorithms and Rust.

## Warning

The I/O APIs are not stable.  Breaking changes may be made in the versions `0.y.z`.
The current versions are good for learning, though.

## Features

- In this repo, there is **NO** code for the exercises and assignments.
- The library code only depends on Rust standard library, no other library is
  used.
- Complete documentation.
- Unit tests in the `tests` modules.
- Clients and utilities in `examples` directory.
- Big-O analysis tools: Doubling Test, Doubling Ratio, Stop Watch (using Rust
  std), in `examples` directory, and `LinearRegression` in a library module.
- Standard Input `StdIn`, file input `FileIn`, and abstract versatile input
  `In`, backed by the Java-util-like `Scanner`.  They are all implemented
  without any external dependency.
- Simplified but professional implementation of vector `SVec` to demonstrate
  Rust's equivalences of `ResizingArrayStack`, `ResizingArrayQueue`, etc.
  Advanced Rust is demonstrated in `SVec`.
- Three implementations of Queue, with discussions, regarding linked list and
  vector.

## How to

```
cargo build
cargo build --examples  # build all the example binaries
cargo test
cargo test -- --show-output   # show stdout
cargo test bst   # only run the tests which contain name "bst"
cargo run --example doubling_ratio  # only run the binary doubling_ratio
```

Most internal modules are re-exported.  For example,

```
use algs4_rs::BST; // `algs4_rs::bst::BST` is used
```

## Copyright

Copyright &copy; 2025 by Rui Duan.

## License

This code is released under GPL version 3 or later.
