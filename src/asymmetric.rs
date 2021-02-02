extern crate sodiumoxide;
use sodiumoxide::crypto::box_;

pub struct Asymmetric {}

impl Asymmetric {

    pub fn key_pair() -> ([u8; 32], [u8; 32]) {
        let (pk, sk) = box_::gen_keypair();
        (pk.0, sk.0)
    }

    fn to_secret_key(sk: &mut [u8;32]) -> box_::SecretKey {
        box_::SecretKey(*sk)
    }

    fn to_public_key(pk: &mut [u8;32]) -> box_::PublicKey {
        box_::PublicKey(*pk)
    }

    pub fn public(sk: &mut [u8; 32]) -> [u8; 32] {

        let to_secret = Asymmetric::to_secret_key(sk);
        to_secret.public_key().0
    }

    pub fn encrypt(msg: &[u8], pk: &mut [u8;32], sk: &mut [u8; 32]) -> Vec<u8> {

        let nonce = box_::gen_nonce();
        let ct = box_::seal(
            msg,
            &nonce,
            &Asymmetric::to_public_key(pk),
            &Asymmetric::to_secret_key(sk)
        );

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

    pub fn decrypt(ct: &[u8], pk: &mut [u8; 32], sk: &mut [u8; 32]) -> Result<Vec<u8>, ()> {

        let nonce = Asymmetric::strip_nonce_from_ct(ct);
        let ct = &ct[24..];
        box_::open(
            ct,
            &nonce,
            &Asymmetric::to_public_key(pk),
            &Asymmetric::to_secret_key(sk)
        )
    }
}
