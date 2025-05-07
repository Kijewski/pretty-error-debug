// SPDX-License-Identifier: MIT OR Apache-2.0

//! # pretty-error-debug
//!
//! [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/pretty-error-debug/ci.yml?branch=main&logo=github&style=flat-square)](https://github.com/Kijewski/pretty-error-debug/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/pretty-error-debug?logo=rust&style=flat-square)](https://crates.io/crates/pretty-error-debug)
//! ![Minimum supported Rust version: 1.56](https://img.shields.io/badge/rustc-1.56+-important?logo=rust&style=flat-square "Minimum Supported Rust Version: 1.56")
//! [![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-informational?logo=apache&style=flat-square)](/LICENSE-MIT "License: MIT OR Apache-2.0")
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
//!     fn source(&self) -> Option<&(dyn 'static + Error)> {
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

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[rustversion::before(1.81.0)]
extern crate std;
#[rustversion::since(1.81.0)]
use core as std;

mod implementation;
#[cfg(test)]
mod test;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use core;
use core::fmt;
use std::error::Error;

#[cfg(feature = "derive")]
pub use pretty_error_debug_derive::PrettyDebug as Debug;

pub use self::implementation::pretty_error_debug;

/// Wrap an [`Error`] to display its error chain in debug messages ([`format!("{:?}")`][fmt::Debug]).
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
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Wrapper<E: Error + ?Sized + 'static>(pub E);

impl<E: Error + ?Sized + 'static> Wrapper<E> {
    /// Return the wrapped argument.
    #[inline]
    pub const fn new(err: E) -> Self
    where
        E: Sized,
    {
        Self(err)
    }
}

impl<E: Error + 'static> From<E> for Wrapper<E> {
    #[inline]
    fn from(value: E) -> Self {
        Self(value)
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

/// Wrap a reference to an [`Error`] to display its error chain with [`format!("{}")`][fmt::Display].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Display<'a, E: Error + ?Sized>(pub &'a E);

impl<'a, E: Error + ?Sized> Display<'a, E> {
    /// Return the wrapped reference.
    #[inline]
    pub const fn new(err: &'a E) -> Self {
        Self(err)
    }
}

impl<'a, E: Error> From<&'a E> for Display<'a, E> {
    #[inline]
    fn from(value: &'a E) -> Self {
        Self(value)
    }
}

impl<E: Error + ?Sized> fmt::Debug for Display<'_, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Display").field(&self.0).finish()
    }
}

impl<E: Error + ?Sized> fmt::Display for Display<'_, E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pretty_error_debug(&self.0, f)
    }
}
