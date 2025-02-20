use std::ptr::null_mut;

use core_foundation::{
    base::{CFType, CFTypeRef, TCFType}, 
    data::CFData, 
    dictionary::CFDictionary, 
    string::CFString
};

use crate::{
    constants, 
    error::Result,
    ffi::{SecItemAdd, SecItemCopyMatching, SecItemDelete, SecItemUpdate}, 
    os_status::OSStatus,
    Keychain,
};

pub struct KeychainService;

impl KeychainService {
    pub fn init() -> Self {
        Self
    }
}

impl Keychain for KeychainService {
    fn store(&self, service: &str, account: &str, data: &[u8]) -> Result<()> {
        let service: CFString = CFString::new(service);
        let account: CFString = CFString::new(account);
        let value = CFData::from_buffer(data);

        let query: CFDictionary<CFString, CFType> = CFDictionary::from_CFType_pairs(&[
            (CFString::new(constants::SECURITY_KEY_CLASS), CFString::new(constants::SECURITY_KEY_CLASS_GENERIC_PASSWORD).as_CFType()),
            (CFString::new(constants::SECURITY_KEY_ATTRIBUTE_SERVICE), service.as_CFType()),
            (CFString::new(constants::SECURITY_KEY_ATTRIBUTE_ACCOUNT), account.as_CFType()),
            (CFString::new(constants::SECURITY_KEY_VALUE_DATA), value.as_CFType()),
        ]);

        let status: OSStatus = unsafe { SecItemAdd(query.as_concrete_TypeRef(), null_mut()) };

        if status == 0 {
            Ok(())
        } else {
            Err(status.into())
        }
    }

    fn retrieve(&self, service: &str, account: &str) -> Result<Vec<u8>> {
        let service: CFString = CFString::new(service);
        let account: CFString = CFString::new(account);

        let query: CFDictionary<CFString, CFType> = CFDictionary::from_CFType_pairs(&[
            (CFString::new(constants::SECURITY_KEY_CLASS), CFString::new(constants::SECURITY_KEY_CLASS_GENERIC_PASSWORD).as_CFType()),
            (CFString::new(constants::SECURITY_KEY_ATTRIBUTE_SERVICE), service.as_CFType()),
            (CFString::new(constants::SECURITY_KEY_ATTRIBUTE_ACCOUNT), account.as_CFType()),
            (CFString::new(constants::SECURITY_KEY_RETURN_DATA), CFString::new("true").as_CFType()),
            (CFString::new(constants::SECURITY_KEY_MATCH_LIMIT), CFString::new(constants::SECURITY_KEY_MATCH_LIMIT_ONE).as_CFType()),
        ]);

        let mut result: CFTypeRef = null_mut();

        let status: OSStatus = unsafe { SecItemCopyMatching(query.as_concrete_TypeRef(), &mut result) };

        if status == 0 {
            let data = unsafe { CFData::wrap_under_create_rule(result as *mut _) };
            Ok(data.bytes().to_vec())
        } else { 
            Err(status.into())
        }
    }

    fn update(&self, service: &str, account: &str, data: &[u8]) -> Result<()> {
        let service: CFString = CFString::new(service);
        let account: CFString = CFString::new(account);
        let value = CFData::from_buffer(data);

        let query: CFDictionary<CFString, CFType> = CFDictionary::from_CFType_pairs(&[
            (CFString::new(constants::SECURITY_KEY_CLASS), CFString::new(constants::SECURITY_KEY_CLASS_GENERIC_PASSWORD).as_CFType()),
            (CFString::new(constants::SECURITY_KEY_ATTRIBUTE_SERVICE), service.as_CFType()),
            (CFString::new(constants::SECURITY_KEY_ATTRIBUTE_ACCOUNT), account.as_CFType()),
        ]);

        let attributes: CFDictionary<CFString, CFType> = CFDictionary::from_CFType_pairs(&[
            (CFString::new(constants::SECURITY_KEY_VALUE_DATA), value.as_CFType()),
        ]);

        let status: OSStatus = unsafe { SecItemUpdate(query.as_concrete_TypeRef(), attributes.as_concrete_TypeRef()) };

        if status == 0 {
            Ok(())
        } else {
            Err(status.into())
        }
    }

    fn delete(&self, service: &str, account: &str) -> Result<()> {
        let service: CFString = CFString::new(service);
        let account: CFString = CFString::new(account);

        let query: CFDictionary<CFString, CFType> = CFDictionary::from_CFType_pairs(&[
            (CFString::new(constants::SECURITY_KEY_CLASS), CFString::new(constants::SECURITY_KEY_CLASS_GENERIC_PASSWORD).as_CFType()),
            (CFString::new(constants::SECURITY_KEY_ATTRIBUTE_SERVICE), service.as_CFType()),
            (CFString::new(constants::SECURITY_KEY_ATTRIBUTE_ACCOUNT), account.as_CFType()),
        ]);

        let status: OSStatus = unsafe { SecItemDelete(query.as_concrete_TypeRef()) };

        if status == 0 {
            Ok(())
        } else {
            Err(status.into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Result;

    struct MockKeychain;

    impl Keychain for MockKeychain {
        fn store(&self, _service: &str, _account: &str, _data: &[u8]) -> Result<()> {
            Ok(())
        }

        fn retrieve(&self, _service: &str, _account: &str) -> Result<Vec<u8>> {
            Ok(b"mocked-data".to_vec())
        }

        fn update(&self, _service: &str, _account: &str, _data: &[u8]) -> Result<()> {
            Ok(())
        }

        fn delete(&self, _service: &str, _account: &str) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_store() {
        let keychain = MockKeychain;
        let result = keychain.store("test_service", "test_account", b"test_data");
        assert!(result.is_ok());
    }

    #[test]
    fn test_retrieve() {
        let keychain = MockKeychain;
        let result = keychain.retrieve("test_service", "test_account");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"mocked-data");
    }

    #[test]
    fn test_update() {
        let keychain = MockKeychain;
        let result = keychain.update("test_service", "test_account", b"updated_data");
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete() {
        let keychain = MockKeychain;
        let result = keychain.delete("test_service", "test_account");
        assert!(result.is_ok());
    }
}
