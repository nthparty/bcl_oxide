extern crate sodiumoxide;
use sodiumoxide::crypto::box_;

pub struct Asymmetric {}

impl Asymmetric {

    pub fn key_pair() -> (box_::PublicKey, box_::SecretKey) { box_::gen_keypair() }

    pub fn encrypt(msg: &[u8], pk: &box_::PublicKey, sk: &box_::SecretKey) -> Vec<u8> {

        let nonce = box_::gen_nonce();
        let ct = box_::seal(msg, &nonce, pk, sk);

        let mut ret = nonce.0.to_vec();
        ret.extend(ct);
        ret
    }

    fn strip_nonce_from_ct(msg: &[u8]) -> box_::Nonce {

        let mut n: [u8; 24] = [0; 24];
        for i in 0..24 {
            if let Some(j) = msg.get(i) {
                n[i] = *j;
            }
        }
        box_::Nonce(n)
    }

    pub fn decrypt(ct: &[u8], pk: &box_::PublicKey, sk: &box_::SecretKey)
        -> Result<Vec<u8>, ()> {

        let nonce = Asymmetric::strip_nonce_from_ct(ct);
        let ct = &ct[24..];
        box_::open(ct, &nonce, pk, sk)
    }
}
