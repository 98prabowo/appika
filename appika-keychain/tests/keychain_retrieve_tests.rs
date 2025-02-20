#![cfg(target_os = "macos")]

mod common;

use appika_keychain::{Keychain, KeychainService};
use common::keychain_cleanup::KeychainCleanup;

#[test]
fn test_keychain_store_and_retrieve() {
    let keychain = KeychainService::init();
    let service = "com.example.app.test.success";
    let account = "test@retrieve.com";
    let data = b"secure-data";

    let _cleanup = KeychainCleanup::new(&keychain, service, account);

    keychain.store(service, account, data).unwrap();

    let retrieved = keychain.retrieve(service, account).unwrap();

    assert_eq!(retrieved, data);
}

#[test]
fn test_keychain_retrieve_non_existent_item() {
    let service = "com.example.app.test.empty";
    let account = "test@retrieve.com";
    let keychain = KeychainService::init();
    let result = keychain.retrieve(service, account);

    assert!(result.is_err());
}

#[test]
fn test_keychain_retrieve_data_mismatch() {
    let keychain = KeychainService::init();
    let service = "com.example.app.test.missmatch";
    let account = "test@retrieve.com";

    let data1 = b"data1";
    let data2 = b"data2";

    let _cleanup = KeychainCleanup::new(&keychain, service, account);

    keychain.store(service, account, data1).unwrap();

    let retrieved = keychain.retrieve(service, account).unwrap();

    assert_ne!(retrieved, data2, "Retrieved data should not match incorrect data");
}

