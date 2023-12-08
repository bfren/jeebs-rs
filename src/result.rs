use std::io::{stderr, Write};

use crate::error::Error;

/**
# Results type
*/
pub type Result<T> = core::result::Result<T, Error>;

pub trait Handle<T> {
    fn handle(self) -> T;
}

impl<T> Handle<T> for Result<T> {
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
