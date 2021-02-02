extern crate sodiumoxide;
use sodiumoxide::crypto::secretbox;

pub struct Symmetric {}

impl Symmetric {

    pub fn secret() -> [u8; 32] {
        let sk = secretbox::gen_key();
        sk.0
    }

    fn to_secret_key(sk: &mut [u8; 32]) -> secretbox::Key {
        secretbox::Key(*sk)
    }

    pub fn encrypt(msg: &[u8], k: &mut [u8;32]) -> Vec<u8> {

        let nonce = secretbox::gen_nonce();
        let ct = secretbox::seal(
            msg,
            &nonce,
            &Symmetric::to_secret_key(k)
        );

        let mut ret = nonce.0.to_vec();
        ret.extend(ct);
        ret
    }

    fn strip_nonce_from_ct(ct: &[u8]) -> secretbox::Nonce {

        let mut n: [u8; 24] = [0; 24];
        for i in 0..24 {
            if let Some(j) = ct.get(i) {
                n[i] = *j;
            }
        }
        secretbox::Nonce(n)
    }

    pub fn decrypt(ct: &[u8], k: &mut [u8;32]) -> Result<Vec<u8>, ()> {

        let nonce = Symmetric::strip_nonce_from_ct(ct);
        let ct = &ct[24..];
        secretbox::open(
            ct,
            &nonce,
            &Symmetric::to_secret_key(k)
        )
    }
}