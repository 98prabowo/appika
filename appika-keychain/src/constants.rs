use core_foundation::{base::TCFType, string::CFString};
use security_framework_sys::item::*;

pub fn security_key_class() -> CFString { 
    unsafe { CFString::wrap_under_get_rule(kSecClass) }
}

pub fn security_key_class_generic_password() -> CFString { 
    unsafe { CFString::wrap_under_get_rule(kSecClassGenericPassword) }
}

pub fn security_key_attribute_service() -> CFString { 
    unsafe { CFString::wrap_under_get_rule(kSecAttrService) }
}

pub fn security_key_attribute_account() -> CFString { 
    unsafe { CFString::wrap_under_get_rule(kSecAttrAccount) }
}

pub fn security_key_value_data() -> CFString { 
    unsafe { CFString::wrap_under_get_rule(kSecValueData) }
}

pub fn security_key_return_data() -> CFString {
    unsafe { CFString::wrap_under_get_rule(kSecReturnData) }
}

pub fn security_key_match_limit() -> CFString {
    unsafe { CFString::wrap_under_get_rule(kSecMatchLimit) }
}
