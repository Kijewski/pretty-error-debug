# pretty-error-debug

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/pretty-error-debug/ci.yml?branch=main&logo=github)](https://github.com/Kijewski/pretty-error-debug/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/pretty-error-debug?logo=rust)](https://crates.io/crates/pretty-error-debug)
![Minimum supported Rust version: 1.31](https://img.shields.io/badge/rustc-1.31+-important?logo=rust "Minimum Supported Rust Version: 1.31")
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-informational?logo=apache)](/LICENSE-MIT "License: MIT OR Apache-2.0")

Display a the chain of an error. Most useful as `Result<(), E>` for your `fn main()`,
and in conjunction with [`thiserror`](https://crates.io/crates/thiserror).

This crate simply <del>plagiarized</del> <ins>extracted</ins> all the relevant formatting code from
[`anyhow`](https://crates.io/crates/anyhow).

```rust
#[derive(pretty_error_debug::Debug, thiserror::Error)]
pub enum MyError {
    #[error("Error variant 1 happened")]
    Variant1(#[from] Error1),
    #[error("Error variant 2 happened")]
    Variant2(#[from] Error2),
}

fn main() -> Result<(), MyError> {
    …
}
```
