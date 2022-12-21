# pretty-error-debug

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
