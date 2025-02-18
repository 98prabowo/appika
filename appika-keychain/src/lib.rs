mod constants;
mod error;
mod ffi;
mod os_status;
mod service;

pub use error::{Result, Error};
pub use service::KeychainService;

pub trait Keychain {
    fn store_password(&self, service: &str, account: &str, password: &str) -> error::Result<()>;
    fn get_password(&self, service: &str, account: &str) -> error::Result<String>;
    fn update_password(&self, service: &str, account: &str, new_password: &str) -> error::Result<()>;
    fn delete_password(&self, service: &str, account: &str) -> error::Result<()>;
}
