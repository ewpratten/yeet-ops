# The `yeet!` macro
[![Crates.io](https://img.shields.io/crates/v/yeet-ops)](https://crates.io/crates/yeet-ops) 
[![Docs.rs](https://docs.rs/yeet-ops/badge.svg)](https://docs.rs/yeet-ops) 
[![Build](https://github.com/Ewpratten/yeet-ops/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/yeet-ops/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/yeet-ops/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/yeet-ops/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/yeet-ops/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/yeet-ops/actions/workflows/audit.yml)


`yeet-ops` is a super small crate to compliment the [`do yeet`](https://github.com/rust-lang/rust/issues/96373) statement implemented in https://github.com/rust-lang/rust/pull/96376

This crate **requires** a nightly build of Rust.

## Usage

```rust
#![feature(yeet_expr)] // Needed!
use yeet_ops::yeet;

/// A function that yeets `None`
fn test() -> Option<i32> {
    yeet!();
}

fn main() {
    // Did it yeet?
    assert_eq!(test(), None);
}
```
