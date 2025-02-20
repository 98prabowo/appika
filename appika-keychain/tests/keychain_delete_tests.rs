#![cfg(target_os = "macos")]

mod common;

use appika_keychain::{Keychain, KeychainService};
use common::keychain_cleanup::KeychainCleanup;

#[test]
fn test_keychain_store_and_delete() {
    let service = "com.example.app.test.success";
    let account = "test@delete.com";
    let keychain = KeychainService::init();
    let data = b"secure-data";

    let _cleanup = KeychainCleanup::new(&keychain, service, account);

    keychain.store(service, account, data).unwrap();

    let result = keychain.delete(service, account);

    assert!(result.is_ok());
}

#[test]
fn test_keychain_delete_non_existent_item() {
    let service = "com.example.app.test.empty";
    let account = "test@delete.com";
    let keychain = KeychainService::init();
    let result = keychain.delete(service, account);

    assert!(result.is_err());
}

#[test]
fn test_keychain_deleted_entry_is_not_retrievable() {
    let service = "com.example.app.test.leaked";
    let account = "test@delete.com";
    let keychain = KeychainService::init();
    let data = b"secure-data";

    let _cleanup = KeychainCleanup::new(&keychain, service, account);

    keychain.store(service, account, data).unwrap();

    keychain.delete(service, account).unwrap();

    let result = keychain.retrieve(service, account);

    assert!(result.is_err(), "Deleted entry should not be retrievable");
}
