# bcl_oxide

Rust library that provides a simple interface for symmetric (i.e., secret-key) 
and asymmetric (i.e., public-key) encryption/decryption primitives. 

## Purpose
This library is a reimplementation of [bcl](https://github.com/nthparty/bcl), 
and is designed to provide the same functionalities for applications written in Rust.

## Package Installation

This package requires [sodiumoxide](https://github.com/sodiumoxide/sodiumoxide), which
must be installed manually (`cargo install` can find the package, but can't install for 
some reason, I'm trying to figure out why). Once installed, edit the `path` variable in 
this repository's `Cargo.toml` file to point to the root directory of `sodiumoxide`.

## Examples

### Symmetric

```rust
extern crate bcl_oxide;
use bcl_oxide::symmetric::Symmetric;
use std::str;

fn main() {
    
    let k = Symmetric::secret();
    println!("Key: {:?}", k);

    let msg = String::from("Hello");
    println!("Msg: {}", msg);
    let ct = Symmetric::encrypt(&msg.into_bytes(), &k);
    println!("Ciphertext: {:?}", ct);

    let ot = Symmetric::decrypt(&ct, &k).unwrap();
    let pt = match str::from_utf8(&ot) {
        Ok(v) => v,
        _ => "err"
    };
    println!("Decrypted: {}", pt);
}

/*
Output:
Key: Key(****)
Msg: Hello
Ciphertext: [107, 91, 51, 83, 63, 0, 12, 103, 33, 131, 168, 188, 77, 146, 33, 254, 8, 13, 67, 80, 119, 136, 7, 147, 134, 165, 154, 188, 134, 35, 242, 1, 255, 254, 90, 34, 23, 223, 140, 76, 140, 21, 6, 70, 51]
Decrypted: Hello
 */
```

### Asymmetric

```rust
extern crate bcl_oxide;
use bcl_oxide::symmetric::Asymmetric;
use std::str;

fn main() {
    
    let (mypk, mysk) = Asymmetric::key_pair();
    println!("My keys: {:?} {:?}", mypk, mysk.0);
    let (otherpk, othersk) = Asymmetric::key_pair();
    println!("Their keys: {:?} {:?}", otherpk, othersk);

    let msg = String::from("Hello");
    println!("Msg: {}", msg);
    let ct = Asymmetric::encrypt(&msg.into_bytes(), &otherpk, &mysk);
    println!("Ciphertext: {:?}", ct);

    let ot = Asymmetric::decrypt(&ct, &mypk, &othersk).unwrap();
    let pt = match str::from_utf8(&ot) {
        Ok(v) => v,
        _ => "err"
    };
    println!("Decrypted: {}", pt);
}

/*
Output:
My keys: PublicKey([129, 165, 84, 54, 216, 8, 150, 143, 177, 172, 79, 252, 209, 185, 208, 127, 112, 242, 99, 222, 138, 78, 139, 114, 124, 172, 247, 173, 88, 54, 215, 71]) [59, 66, 70, 230, 67, 180, 175, 72, 179, 44, 157, 130, 113, 254, 13, 151, 81, 152, 36, 236, 121, 175, 249, 244, 95, 144, 197, 44, 146, 173, 76, 57]
Their keys: PublicKey([66, 45, 142, 180, 165, 59, 78, 249, 159, 38, 78, 29, 24, 248, 108, 98, 10, 52, 228, 223, 105, 167, 14, 13, 200, 140, 57, 176, 128, 243, 60, 53]) SecretKey(****)
Msg: Hello
Ciphertext: [186, 120, 72, 228, 251, 123, 25, 166, 226, 165, 32, 88, 200, 169, 116, 113, 97, 176, 21, 219, 214, 242, 82, 67, 110, 161, 95, 158, 16, 183, 242, 218, 209, 251, 102, 68, 151, 249, 188, 57, 208, 54, 249, 17, 101]
Decrypted: Hello
 */
```

Alternatively, you generate key pairs from a supplied 32-byte array:

```rust
    let mut my_sk: [u8; 32] = [0; 32];
    let my_sk = Asymmetric::to_secret(&mut my_sk);
    let my_pk = Asymmetric::public(&sk);

    let mut their_sk: [u8; 32] = [0; 32];
    let their_sk = Asymmetric::to_secret(&mut their_sk);
    let my_pk = Asymmetric::public(&their_sk);
```
