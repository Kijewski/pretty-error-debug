// ////////////////////////////////////////////////////////////////////////////////////////////////
// All code in this file was extracted from:
//  * https://github.com/dtolnay/anyhow/blob/0ba6408b5ef508c3dfc95797d21cfbdca9dd64ee/src/fmt.rs
//  * https://github.com/dtolnay/anyhow/blob/fa9bcc0457a2e51593b874cc2f8bcb5608ad43fe/src/chain.rs
//
// Author: David Tolnay <dtolnay@gmail.com> and contributors to the `anyhow` project.
// ////////////////////////////////////////////////////////////////////////////////////////////////

use core::fmt::{self, Write};

use crate::Error;

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
pub fn pretty_error_debug(error: &dyn Error, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(error, f)?;
    if let Some(cause) = error.source() {
        f.write_str("\n\nCaused by:")?;
        let multiple = cause.source().is_some();
        for (n, error) in Chain(Some(cause)).enumerate() {
            f.write_char('\n')?;
            let mut indented = Indented {
                inner: f,
                number: if multiple { Some(n + 1) } else { None },
                started: false,
            };
            write!(indented, "{error}")?;
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

struct Indented<'a, 'b> {
    inner: &'a mut fmt::Formatter<'b>,
    number: Option<usize>,
    started: bool,
}

impl fmt::Write for Indented<'_, '_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (i, line) in s.split('\n').enumerate() {
            if !self.started {
                self.started = true;
                match self.number {
                    Some(number) => write!(self.inner, "{number: >5}: ")?,
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
