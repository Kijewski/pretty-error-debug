// SPDX-License-Identifier: MIT OR Apache-2.0

//! # pretty-error-debug
//!
//! [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/pretty-error-debug/ci.yml?branch=main&logo=github)](https://github.com/Kijewski/pretty-error-debug/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/pretty-error-debug?logo=rust)](https://crates.io/crates/pretty-error-debug)
//! ![Minimum supported Rust version: 1.60](https://img.shields.io/badge/rustc-1.60+-important?logo=rust "Minimum Supported Rust Version: 1.60")
//! [![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-informational?logo=apache)](/LICENSE-MIT "License: MIT OR Apache-2.0")
//!
//! Display a the chain of an error. Most useful as [`Result<(), E>`](std::result::Result) for your `fn main()`,
//! and in conjunction with [`thiserror`](https://crates.io/crates/thiserror).
//!
//! This crate simply <del>plagiarized</del> <ins>extracted</ins> all the relevant formatting code from
//! [`anyhow`](https://crates.io/crates/anyhow).
//!
//! ## Example message
//!
//! ```text
//! Error: Got a 'middle' error
//!
//! Caused by:
//!     1: A nested error occured
//!     2: 'inner' failed
//!     3: Caught an error: Not implemented, yet.
//! ```
//!
//! ## With `thiserror`
//!
//! ```rust,ignore
//! #[derive(pretty_error_debug::Debug, thiserror::Error)]
//! pub enum MyError {
//!     #[error("Error variant 1 happened")]
//!     Variant1(#[from] Error1),
//!     #[error("Error variant 2 happened")]
//!     Variant2(#[from] Error2),
//! }
//!
//! fn main() -> Result<(), MyError> {
//! # /*
//!     ...
//! # */ Ok(())
//! }
//! ```
//!
//! ## With `thiserror`, but without a new type
//!
//! ```rust,ignore
//! #[derive(Debug, thiserror::Error)]
//! pub enum MyError {
//!     #[error("Error variant 1 happened")]
//!     Variant1(#[from] Error1),
//!     #[error("Error variant 2 happened")]
//!     Variant2(#[from] Error2),
//! }
//!
//! fn main() -> Result<(), pretty_error_debug::Wrapper<MyError>> {
//! # /*
//!     ...
//! # */ Ok(())
//! }
//! ```
//!
//! ## Without `thiserror`
//!
//! ```rust
//! use std::error::Error;
//! use std::fmt::{self, Write};
//! #
//! # #[derive(Debug)] pub struct Error1;
//! # impl Error for Error1 {}
//! # impl fmt::Display for Error1 {
//! #    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//! #        write!(f, "Error1")
//! #    }
//! # }
//! #
//! # #[derive(Debug)] pub struct Error2;
//! # impl Error for Error2 {}
//! # impl fmt::Display for Error2 {
//! #    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//! #        write!(f, "Error2")
//! #    }
//! # }
//!
//! #[derive(pretty_error_debug::Debug)]
//! pub enum MyError {
//!     Variant1(Error1),
//!     Variant2(Error2),
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
//! fn main() -> Result<(), MyError> {
//! # /*
//!     ...
//! # */ Ok(())
//! }
//! ```
//!

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(absolute_paths_not_starting_with_crate)]
#![warn(elided_lifetimes_in_paths)]
#![warn(meta_variable_misuse)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(unused_extern_crates)]
#![warn(unused_lifetimes)]
#![warn(unused_results)]

#[cfg(test)]
mod test;

use std::error::Error;
use std::fmt;
use std::fmt::Write;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(inline, cfg(feature = "derive")))]
pub use pretty_error_debug_derive::PrettyDebug as Debug;

/// Instead of adding a new type, you can simply use this wrapper
///
/// ## Example
///
/// ```rust
/// use some_external_mod::{SomeError, some_test};
///
/// fn main() -> Result<(), pretty_error_debug::Wrapper<SomeError>> {
///     some_test()?;
///     Ok(())
/// }
///
/// mod some_external_mod {
///     #[derive(Debug)]
///     pub struct SomeError;
///
///     impl std::error::Error for SomeError {}
///
///     impl std::fmt::Display for SomeError {
///         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///             f.write_str("Something went wrong")
///         }
///     }
///
///     pub fn some_test() -> Result<(), SomeError> {
/// # /*
///         Err(SomeError)
/// # */ Ok(())
///     }
/// }
/// ```
#[derive(Clone, Copy, Default)]
pub struct Wrapper<E: Error + 'static>(E);

impl<E: Error + 'static> From<E> for Wrapper<E> {
    #[inline]
    fn from(value: E) -> Self {
        Wrapper(value)
    }
}

impl<E: Error + 'static> Error for Wrapper<E> {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl<E: Error + 'static> fmt::Display for Wrapper<E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<E: Error + 'static> fmt::Debug for Wrapper<E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pretty_error_debug(&self.0, f)
    }
}

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
pub fn pretty_error_debug(error: &dyn Error, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
