use crate::os_status::{OSStatus, error_description};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    OSStatusError(OSStatus),
    NotFound,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self {
            Self::OSStatusError(status) => {
                let desc: String = error_description(*status);
                write!(fmt, "{desc}")
            }
            Self::NotFound => write!(fmt, "Item not found"),
        }
    }
}

impl From<OSStatus> for Error {
    fn from(value: OSStatus) -> Self {
        if value == -25300 { 
            Self::NotFound
        } else { 
            Self::OSStatusError(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_status_error() {
        let err = Error::OSStatusError(1);
        assert_eq!(format!("{}", err), "Operation failed with OS status code 1: Unspecified error")
    }

    #[test]
    fn test_keychain_not_found() {
        let err = Error::NotFound;
        assert_eq!(format!("{}", err), "Item not found")
    }
}
