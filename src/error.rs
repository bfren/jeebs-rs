use std::{any::type_name, fmt::{self, Display}};

#[derive(Debug)]
pub struct Error {
    value: Box<dyn std::error::Error + Send + Sync + 'static>,
    kind: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} [{}]", self.value, self.kind)
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(value: E) -> Self {
        Error {
            value: Box::new(value),
            kind: type_name::<E>().to_string(),
        }
    }
}
