use std::error::Error;
use std::fmt;

#[derive(crate::Debug, Clone, Copy)]
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

#[derive(crate::Debug, Clone, Copy)]
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

#[derive(crate::Debug, Clone, Copy)]
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

fn root() -> Result<(), RootError> {
    Err(RootError::Reasons)
}

fn inner() -> Result<(), InnerError> {
    root().map_err(InnerError::from)
}

fn outer() -> Result<(), OuterError> {
    inner().map_err(OuterError::from)
}

#[test]
fn test() {
    const EXPECTED: &str = "\
Got an InnerError

Caused by:
    1: Failed because of Reasons
    2: Reasons\
";

    let outcome = format!("{:?}", outer().unwrap_err());
    assert_eq!(EXPECTED, &outcome);
}
