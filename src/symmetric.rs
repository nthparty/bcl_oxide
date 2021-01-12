extern crate sodiumoxide;
use sodiumoxide::crypto::secretbox;

pub struct Symmetric {}

impl Symmetric {

    pub fn secret() -> secretbox::Key { secretbox::gen_key() }

    pub fn nonce() -> secretbox::Nonce { secretbox::gen_nonce() }

    pub fn encrypt(msg: &[u8], n: &secretbox::Nonce, k: &secretbox::Key)
               -> Vec<u8> { secretbox::seal(msg, n, k) }

    pub fn decrypt(ct: &[u8], n: &secretbox::Nonce, k: &secretbox::Key)
               -> Result<Vec<u8>, ()> { secretbox::open(ct, n, k) }
}