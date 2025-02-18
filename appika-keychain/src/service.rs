use std::{
    ptr::null_mut, str::from_utf8
};

use core_foundation::{
    base::{CFTypeRef, TCFType}, 
    dictionary::{CFDictionary, CFDictionaryRef}, 
    string::{CFString, CFStringRef}
};

use crate::{
    constants, 
    error,
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
    fn store_password(&self, service: &str, account: &str, password: &str) -> error::Result<()> {
        let service: CFString = CFString::new(service);
        let account: CFString = CFString::new(account);
        let password: &[u8] = password.as_bytes();
        let password = from_utf8(password)?;

        let keys: Vec<CFString> = vec![
            CFString::new(constants::SECURITY_KEY_CLASS),
            CFString::new(constants::SECURITY_KEY_ATTRIBUTE_SERVICE),
            CFString::new(constants::SECURITY_KEY_ATTRIBUTE_ACCOUNT),
            CFString::new(constants::SECURITY_KEY_VALUE_DATA),
        ];

        let values: Vec<CFString> = vec![
            CFString::new(constants::SECURITY_KEY_CLASS_GENERIC_PASSWORD),
            service,
            account,
            CFString::new(password),
        ];

        let query: CFDictionaryRef = make_dictionary(keys, values).as_concrete_TypeRef();

        let status: OSStatus = unsafe { SecItemAdd(query, null_mut()) };

        if status == 0 {
            Ok(())
        } else {
            Err(status.into())
        }
    }

    fn get_password(&self, service: &str, account: &str) -> error::Result<String> {
        let service: CFString = CFString::new(service);
        let account: CFString = CFString::new(account);

        let keys: Vec<CFString> = vec![
            CFString::new(constants::SECURITY_KEY_CLASS),
            CFString::new(constants::SECURITY_KEY_ATTRIBUTE_SERVICE),
            CFString::new(constants::SECURITY_KEY_ATTRIBUTE_ACCOUNT),
            CFString::new(constants::SECURITY_KEY_RETURN_DATA),
            CFString::new(constants::SECURITY_KEY_MATCH_LIMIT),
        ];

        let values: Vec<CFString> = vec![
            CFString::new(constants::SECURITY_KEY_CLASS_GENERIC_PASSWORD),
            service,
            account,
            CFString::new("true"),
            CFString::new(constants::SECURITY_KEY_MATCH_LIMIT_ONE),
        ];

        let query: CFDictionaryRef = make_dictionary(keys, values).as_concrete_TypeRef();

        let mut result: CFTypeRef = null_mut();

        let status: OSStatus = unsafe { SecItemCopyMatching(query, &mut result) };

        if status == 0 {
            let data = unsafe { CFString::wrap_under_create_rule(result as CFStringRef) };
            Ok(data.to_string())
        } else {
            Err(status.into())
        }
    }

    fn update_password(&self, service: &str, account: &str, new_password: &str) -> error::Result<()> {
        let service: CFString = CFString::new(service);
        let account: CFString = CFString::new(account);
        let new_pasword: &[u8] = new_password.as_bytes();
        let new_password: &str = from_utf8(new_pasword)?;

        let keys: Vec<CFString> = vec![
            CFString::new(constants::SECURITY_KEY_CLASS),
            CFString::new(constants::SECURITY_KEY_ATTRIBUTE_SERVICE),
            CFString::new(constants::SECURITY_KEY_ATTRIBUTE_ACCOUNT),
        ];

        let values: Vec<CFString> = vec![
            CFString::new(constants::SECURITY_KEY_CLASS_GENERIC_PASSWORD),
            service,
            account,
        ];

        let query: CFDictionaryRef = make_dictionary(keys, values).as_concrete_TypeRef();

        let update_keys: Vec<CFString> = vec![CFString::new(constants::SECURITY_KEY_VALUE_DATA)];
        let update_values: Vec<CFString> = vec![CFString::new(new_password)];
        let update_attr: CFDictionaryRef = make_dictionary(update_keys, update_values).as_concrete_TypeRef();

        let status: OSStatus = unsafe { SecItemUpdate(query, update_attr) };

        if status == 0 {
            Ok(())
        } else {
            Err(status.into())
        }
    }

    fn delete_password(&self, service: &str, account: &str) -> error::Result<()> {
        let service: CFString = CFString::new(service);
        let account: CFString = CFString::new(account);

        let keys: Vec<CFString> = vec![
            CFString::new(constants::SECURITY_KEY_CLASS),
            CFString::new(constants::SECURITY_KEY_ATTRIBUTE_SERVICE),
            CFString::new(constants::SECURITY_KEY_ATTRIBUTE_ACCOUNT),
        ];

        let values: Vec<CFString> = vec![
            CFString::new(constants::SECURITY_KEY_CLASS_GENERIC_PASSWORD),
            service,
            account,
        ];

        let query: CFDictionaryRef = make_dictionary(keys, values).as_concrete_TypeRef();

        let status: OSStatus = unsafe { SecItemDelete(query) };

        if status == 0 {
            Ok(())
        } else {
            Err(status.into())
        }
    }
}

fn make_dictionary(keys: Vec<CFString>, values: Vec<CFString>) -> CFDictionary<CFString, CFString> {
    let kv_pairs: &Vec<(CFString, CFString)> = &keys.into_iter()
        .zip(values)
        .collect();

    CFDictionary::from_CFType_pairs(kv_pairs)
}
