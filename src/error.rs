use std::{
    any::type_name,
    fmt::{self, Display},
};

#[derive(Debug, PartialEq)]
pub struct Error {
    pub value: String,
    pub kind: Option<&'static str>,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            Some(k) => write!(f, "{} [{}]", self.value, k),
            None => write!(f, "{}", self.value)
        }
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(value: E) -> Self {
        Error {
            value: value.to_string(),
            kind: Some(type_name::<E>()),
        }
    }
}

pub trait AsError {
    fn as_error(self) -> Error;
}

impl<T> AsError for T
where
    T: ToString
{
    fn as_error(self) -> Error {
        Error {
            value: self.to_string(),
            kind: None
        }
    }
}
