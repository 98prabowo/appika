#![cfg(target_os = "macos")]

mod common;

use appika_keychain::{Keychain, KeychainService};
use common::keychain_cleanup::KeychainCleanup;

#[test]
fn test_keychain_store_success() {
    let keychain = KeychainService::init();
    let service = "com.example.app.test.success";
    let account = "test@store.com";
    let data = b"secure-data";

    let _cleanup = KeychainCleanup::new(&keychain, service, account);

    let stored = keychain.store(service, account, data);

    assert!(stored.is_ok());
}

#[test]
fn test_keychain_store_twice() {
    let keychain = KeychainService::init();
    let service = "com.example.app.test.twice";
    let account = "test@store.com";
    let data = b"secure-data";

    let _cleanup = KeychainCleanup::new(&keychain, service, account);

    let first_store = keychain.store(service, account, data);

    assert!(first_store.is_ok());

    let second_store = keychain.store(service, account, data);

    assert!(second_store.is_err());
}
