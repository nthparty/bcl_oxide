extern crate sodiumoxide;
use sodiumoxide::crypto::box_;

pub struct Asymmetric {}

impl Asymmetric {

    pub fn secret() -> (box_::PublicKey, box_::SecretKey) {
        box_::gen_keypair()
    }

    pub fn nonce() -> box_::Nonce {
        box_::gen_nonce()
    }

    pub fn encrypt(msg: &[u8], n: &box_::Nonce, pk: &box_::PublicKey, sk: &box_::SecretKey)
               -> Vec<u8>
    {
        box_::seal(msg, n, pk, sk)
    }

    pub fn decrypt(ct: &[u8], n: &box_::Nonce, pk: &box_::PublicKey, sk: &box_::SecretKey)
               -> Result<Vec<u8>, ()> { box_::open(ct, n, pk, sk) }
}
