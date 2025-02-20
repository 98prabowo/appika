#![cfg(target_os = "macos")]

mod common;

use appika_keychain::{Keychain, KeychainService};
use common::keychain_cleanup::KeychainCleanup;

#[test]
fn test_keychain_update_success() {
    let keychain = KeychainService::init();
    let service = "com.example.app.test.success";
    let account = "test@update.com";
    let old_data = b"old-data";
    let new_data = b"new-data";

    let _cleanup = KeychainCleanup::new(&keychain, service, account);

    keychain.store(service, account, old_data).unwrap();

    let retrieved = keychain.retrieve(service, account).unwrap();

    assert_eq!(retrieved, old_data);

    keychain.update(service, account, new_data).unwrap();

    let retrieved = keychain.retrieve(service, account).unwrap();

    assert_eq!(retrieved, new_data);
}

#[test]
fn test_keychain_update_non_existent_item() {
    let keychain = KeychainService::init();
    let service = "com.example.app.test.empty";
    let account = "test@update.com";
    let new_data = b"new-data";

    let _cleanup = KeychainCleanup::new(&keychain, service, account);

    let updated = keychain.update(service, account, new_data);

    assert!(updated.is_err());
}
