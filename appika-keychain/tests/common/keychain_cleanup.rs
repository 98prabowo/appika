use appika_keychain::Keychain;

pub struct KeychainCleanup<'a, K: Keychain> {
    keychain: &'a K,
    service: &'a str,
    account: &'a str,
}

impl<'a, K> KeychainCleanup<'a, K>
where 
    K: Keychain
{
    pub fn new(keychain: &'a K, service: &'a str, account: &'a str) -> Self {
        let _ = keychain.delete(service, account);
        Self { keychain, service, account }
    }
}

impl<'a, K> Drop for KeychainCleanup<'a, K>
where 
    K: Keychain
{
    fn drop(&mut self) {
        let _ = self.keychain.delete(self.service, self.account);
    }
}
