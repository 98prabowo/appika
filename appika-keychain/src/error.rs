use std::str::Utf8Error;

use crate::os_status::{OSStatus, error_description};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    OSError(OSStatus),
    StringParseError(Utf8Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OSError(status) => {
                let desc: String = error_description(*status);
                write!(fmt, "{desc}")
            }
            Self::StringParseError(err) => write!(fmt, "{err:?}"),
        }
    }
}

impl From<OSStatus> for Error {
    fn from(value: OSStatus) -> Self {
        Self::OSError(value)
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self::StringParseError(value)
    }
}
