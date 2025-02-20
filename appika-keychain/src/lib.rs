mod constants;
mod error;
mod ffi;
mod os_status;
mod service;

pub use error::{Result, Error};
pub use service::KeychainService;

pub trait Keychain {
    fn store(&self, service: &str, account: &str, data: &[u8]) -> Result<()>;
    fn retrieve(&self, service: &str, account: &str) -> Result<Vec<u8>>;
    fn update(&self, service: &str, account: &str, data: &[u8]) -> Result<()>;
    fn delete(&self, service: &str, account: &str) -> Result<()>;
}
