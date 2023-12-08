use crate::error::Error;
use std::{io::{stderr, Write}, fmt::Display};

/// Alias of [`core::result::Result<T, E>`] to enable capturing errors as an [`Error`] object.
/// 
/// # Examples
/// 
/// ```
/// # use jeebs::error::*;
/// # use jeebs::result::*;
/// 
/// // Set the return value as `Result<T>` and you can use the `?` propagator to capture and return
/// // any errors.  Values have to be captured and returned using `Ok(T)` or the conversion fails.
/// fn parse(value: &str) -> Result<u32> {
///     let result = value.parse::<u32>()?;
///     Ok(result)
/// }
/// 
/// let expected_value = 42;
/// assert_eq!(expected_value, parse("42").unwrap());
/// 
/// let expected_error = Error {
///     value: "invalid digit found in string".to_string(),
///     kind: Some("core::num::error::ParseIntError")
/// };
/// assert_eq!(expected_error, parse("foo").unwrap_err());
/// ```
pub type Result<T> = core::result::Result<T, Error>;

pub trait Handle<T> {
    fn handle(self) -> T;
}

impl<T, E> Handle<T> for core::result::Result<T, E>
where
    E: Display
{
    /// Handles a result object and returns the value if [`Ok(T)`], otherwise prints the error message
    /// and exits the thread.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jeebs::result::*;
    /// let result = "42".parse();
    /// assert_eq!(42, result.handle());
    /// ```
    fn handle(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                let _ = write!(stderr(), "{}", e);
                std::process::exit(1)
            }
        }
    }
}
