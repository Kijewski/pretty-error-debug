mod pretty_error_debug {
    pub use crate::*;
}

use std::error::Error;
use std::fmt;

use crate::{Display, Wrapper};

#[derive(Debug, Clone, Copy)]
enum RootError {
    Reasons,
}

impl fmt::Display for RootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RootError::Reasons => write!(f, "Reasons"),
        }
    }
}

impl Error for RootError {}

#[derive(Debug, Clone, Copy)]
enum InnerError {
    Cause { root: RootError },
}

impl fmt::Display for InnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InnerError::Cause { root } => write!(f, "Failed because of {:?}", root),
        }
    }
}

impl Error for InnerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InnerError::Cause { root } => Some(root),
        }
    }
}

impl From<RootError> for InnerError {
    fn from(root: RootError) -> Self {
        InnerError::Cause { root }
    }
}

#[derive(pretty_error_debug::Debug, Clone, Copy)]
enum OuterError {
    Inner(InnerError),
}

impl fmt::Display for OuterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OuterError::Inner(_) => write!(f, "Got an InnerError"),
        }
    }
}

impl Error for OuterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OuterError::Inner(cause) => Some(cause),
        }
    }
}

impl From<InnerError> for OuterError {
    fn from(cause: InnerError) -> Self {
        OuterError::Inner(cause)
    }
}

#[derive(Debug, Clone, Copy)]
enum SimpleOuterError {
    Inner(InnerError),
}

impl fmt::Display for SimpleOuterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleOuterError::Inner(_) => write!(f, "Got an InnerError"),
        }
    }
}

impl Error for SimpleOuterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SimpleOuterError::Inner(cause) => Some(cause),
        }
    }
}

impl From<InnerError> for SimpleOuterError {
    fn from(cause: InnerError) -> Self {
        SimpleOuterError::Inner(cause)
    }
}

fn root() -> Result<(), RootError> {
    Err(RootError::Reasons)
}

fn inner() -> Result<(), InnerError> {
    root().map_err(InnerError::from)
}

fn outer() -> Result<(), OuterError> {
    inner().map_err(OuterError::from)
}

fn simple_outer() -> Result<(), SimpleOuterError> {
    inner().map_err(SimpleOuterError::Inner)
}

const EXPECTED: &str = "\
Got an InnerError

Caused by:
    1: Failed because of Reasons
    2: Reasons\
";

#[test]
fn test_derive() {
    let outcome = format!("{:?}", outer().unwrap_err());
    assert_eq!(EXPECTED, &outcome);
}

#[test]
fn test_wrapper() {
    let outcome = format!("{:?}", Wrapper::from(simple_outer().unwrap_err()));
    assert_eq!(EXPECTED, &outcome);
}

#[test]
fn test_display() {
    let outcome = format!("{}", Display(&simple_outer().unwrap_err()));
    assert_eq!(EXPECTED, &outcome);
}
