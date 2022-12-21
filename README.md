# pretty-error-debug

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/pretty-error-debug/ci.yml?branch=main&logo=github)](https://github.com/Kijewski/pretty-error-debug/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/pretty-error-debug?logo=rust)](https://crates.io/crates/pretty-error-debug)
![Minimum supported Rust version: 1.30](https://img.shields.io/badge/rustc-1.30+-important?logo=rust "Minimum Supported Rust Version: 1.30")
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-informational?logo=apache)](/LICENSE-MIT "License: MIT OR Apache-2.0")

Display a the chain of an error. Most useful as `Result<(), E>` for your `fn main()`,
and in conjunction with [`thiserror`](https://crates.io/crates/thiserror).

This crate simply <del>plagiarized</del> <ins>extracted</ins> all the relevant formatting code from
[`anyhow`](https://crates.io/crates/anyhow).

```rust
use std::error::Error;
use std::fmt::{self, Write};

pub enum MyError {
    Variant1(Error1),
    Variant2(Error2),
}

impl fmt::Debug for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pretty_error_debug::pretty_error_debug(self, f)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Variant1(_) => write!(f, "Error variant 1 happened"),
            MyError::Variant2(_) => write!(f, "Error variant 2 happened"),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Variant1(source) => Some(source),
            MyError::Variant2(source) => Some(source),
        }
    }
}

fn main() -> Result<(), MyError> {
    …
}
```
