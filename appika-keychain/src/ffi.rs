extern crate libc;
extern crate core_foundation;

use core_foundation::base::CFTypeRef;
use core_foundation::dictionary::CFDictionaryRef;

use crate::os_status::OSStatus;

#[link(name = "Security", kind = "framework")]
extern "C" {
    pub fn SecItemAdd(query: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
    pub fn SecItemCopyMatching(query: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
    pub fn SecItemUpdate(query: CFDictionaryRef, attributesToUpdate: CFDictionaryRef) -> OSStatus;
    pub fn SecItemDelete(query: CFDictionaryRef) -> OSStatus;
}
