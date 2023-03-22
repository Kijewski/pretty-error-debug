/// This example will print the following error message:
///
/// ```text
/// Error: Got a 'middle' error
///
/// Caused by:
///     1: A nested error occured
///     2: 'inner' failed
///     3: Caught an error: Not implemented, yet.
/// ```

fn main() -> Result<(), pretty_error_debug::Wrapper<Outer>> {
    outer()?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Outer {
    #[error("Got a 'middle' error")]
    Middle(#[from] Middle),
}

fn outer() -> Result<(), Outer> {
    middle()?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Middle {
    #[error("A nested error occured")]
    Nested(#[from] Nested),
}

fn middle() -> Result<(), Middle> {
    nested()?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Nested {
    #[error("'inner' failed")]
    Inner(#[from] Inner),
}

fn nested() -> Result<(), Nested> {
    inner()?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[error("Caught an error: {0}")]
struct Inner(&'static str);

fn inner() -> Result<(), Inner> {
    Err(Inner("Not implemented, yet."))
}
