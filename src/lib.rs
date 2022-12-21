// SPDX-License-Identifier: MIT OR Apache-2.0

//! # pretty-error-debug
//!
//! Display a the chain of an error. Most useful as `Result<(), E>` for your `fn main()`,
//! and in conjunction with [`thiserror`](https://crates.io/crates/thiserror).
//!
//! This crate simply <del>plagiarized</del> <ins>extracted</ins> all the relevant formatting code from
//! [`anyhow`](https://crates.io/crates/anyhow).
//!
//! ```
//! use std::error::Error;
//! use std::fmt::{self, Write};
//! #
//! # #[derive(Debug)] struct Error1;
//! # impl Error for Error1 {}
//! # impl fmt::Display for Error1 {
//! #    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//! #        write!(f, "Error1")
//! #    }
//! # }
//! #
//! # #[derive(Debug)] struct Error2;
//! # impl Error for Error2 {}
//! # impl fmt::Display for Error2 {
//! #    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//! #        write!(f, "Error2")
//! #    }
//! # }
//!
//! pub enum MyError {
//!     Variant1(Error1),
//!     Variant2(Error2),
//! }
//!
//! impl fmt::Debug for MyError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         pretty_error_debug::pretty_error_debug(self, f)
//!     }
//! }
//!
//! impl fmt::Display for MyError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         match self {
//!             MyError::Variant1(_) => write!(f, "Error variant 1 happened"),
//!             MyError::Variant2(_) => write!(f, "Error variant 2 happened"),
//!         }
//!     }
//! }
//!
//! impl Error for MyError {
//!     fn source(&self) -> Option<&(dyn Error + 'static)> {
//!         match self {
//!             MyError::Variant1(source) => Some(source),
//!             MyError::Variant2(source) => Some(source),
//!         }
//!     }
//! }
//!
//! # /*
//! fn main() -> Result<(), MyError> {
//!     …
//! }
//! # */
//! ```
//!

#[cfg(test)]
mod test;

use std::error::Error;
use std::fmt;
use std::fmt::Write;

// ////////////////////////////////////////////////////////////////////////////////////////////////
// All further code was extracted from:
//  * https://github.com/dtolnay/anyhow/blob/0ba6408b5ef508c3dfc95797d21cfbdca9dd64ee/src/fmt.rs
//  * https://github.com/dtolnay/anyhow/blob/fa9bcc0457a2e51593b874cc2f8bcb5608ad43fe/src/chain.rs
//
// Author: David Tolnay <dtolnay@gmail.com> and contributors to the `anyhow` project.
// ////////////////////////////////////////////////////////////////////////////////////////////////

/// Write out the [`Error`] message and chain.
///
/// Please see the [`crate`] documentation for a more complete example.
///
/// ```rust
/// use std::fmt::{self, Write};
///
/// pub enum MyError {
///     Variant1(/* … */),
///     Variant2(/* … */),
///     // …
/// }
///
/// impl fmt::Debug for MyError {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/// #       /*
///         pretty_error_debug::pretty_error_debug(self, f)
/// #       */ Ok(())
///     }
/// }
///
/// // TODO: implement `std::fmt::Display` and `std::error::Error`.
/// ```
///
/// # Errors
///
/// Fails if writing to the `f` argument failed.
///
#[cold]
pub fn pretty_error_debug<'a>(error: &dyn Error, f: &mut fmt::Formatter<'a>) -> fmt::Result {
    write!(f, "{}", error)?;
    if let Some(cause) = error.source() {
        write!(f, "\n\nCaused by:")?;
        let multiple = cause.source().is_some();
        for (n, error) in Chain(Some(cause)).enumerate() {
            writeln!(f)?;
            let mut indented = Indented {
                inner: f,
                number: if multiple { Some(n + 1) } else { None },
                started: false,
            };
            write!(indented, "{}", error)?;
        }
    }
    Ok(())
}

struct Chain<'a>(Option<&'a dyn Error>);

impl<'a> Iterator for Chain<'a> {
    type Item = &'a dyn Error;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let error = self.0?;
        self.0 = error.source();
        Some(error)
    }
}

struct Indented<'a, 'b: 'a> {
    inner: &'a mut fmt::Formatter<'b>,
    number: Option<usize>,
    started: bool,
}

impl<'a, 'b: 'a> fmt::Write for Indented<'a, 'b> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (i, line) in s.split('\n').enumerate() {
            if !self.started {
                self.started = true;
                match self.number {
                    Some(number) => write!(self.inner, "{: >5}: ", number)?,
                    None => self.inner.write_str("    ")?,
                }
            } else if i > 0 {
                self.inner.write_char('\n')?;
                if self.number.is_some() {
                    self.inner.write_str("       ")?;
                } else {
                    self.inner.write_str("    ")?;
                }
            }

            self.inner.write_str(line)?;
        }

        Ok(())
    }
}
