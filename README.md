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

    // can also skip this step and supply your own key below
    let mut k = Symmetric::secret();
    println!("Key: {:?}", k);

    let msg = String::from("Hello");
    println!("Msg: {}", msg);
    let ct = Symmetric::encrypt(&msg.into_bytes(), &mut k);
    println!("Ciphertext: {:?}", ct);

    let ot = Symmetric::decrypt(&ct, &mut k).unwrap();
    let pt = match str::from_utf8(&ot) {
        Ok(v) => v,
        _ => "err"
    };
    println!("Decrypted: {}", pt);
}

/*
Output:
Key: [70, 116, 33, 149, 104, 167, 193, 21, 90, 66, 192, 75, 167, 39, 47, 253, 224, 37, 118, 219, 236, 18, 174, 178, 0, 200, 175, 8, 151, 163, 105, 48]
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

    // Can randomly generate keys
    let (mut my_pk, mut my_sk) = Asymmetric::key_pair();
    println!("My keys: \n Public: {:?} \n Secret: {:?}", my_pk, my_sk);

    // Can also generate a public key from a user supplied secret key
    let mut their_sk: [u8; 32] = [0; 32];
    let mut their_pk = Asymmetric::public(&mut their_sk);
    println!("Their keys: \n Public {:?} \n Secret: {:?}", their_pk, their_sk);

    let msg = String::from("Hello");
    println!("Msg: {}", msg);

    let ct = Asymmetric::encrypt(&msg.into_bytes(), &mut their_pk, &mut my_sk);
    println!("Ciphertext: {:?}", ct);

    let ot = Asymmetric::decrypt(&ct, &mut my_pk, &mut their_sk).unwrap();
    let pt = match str::from_utf8(&ot) {
        Ok(v) => v,
        _ => "err"
    };

    println!("Decrypted: {:?}", pt);
}

/*
Output:
My keys: 
 Public: [216, 197, 211, 64, 253, 56, 202, 195, 172, 44, 246, 55, 122, 219, 148, 131, 173, 166, 68, 21, 237, 56, 228, 113, 82, 132, 210, 3, 18, 202, 171, 53] 
 Secret: [183, 127, 7, 27, 151, 201, 121, 207, 240, 253, 116, 64, 13, 134, 205, 240, 133, 36, 211, 75, 33, 84, 47, 158, 142, 211, 220, 53, 122, 226, 157, 39]
Their keys: 
 Public [47, 229, 125, 163, 71, 205, 98, 67, 21, 40, 218, 172, 95, 187, 41, 7, 48, 255, 246, 132, 175, 196, 207, 194, 237, 144, 153, 95, 88, 203, 59, 116] 
 Secret: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
Msg: Hello
Ciphertext: [142, 189, 162, 127, 233, 239, 62, 13, 243, 62, 102, 16, 226, 82, 12, 86, 41, 227, 165, 136, 187, 229, 136, 152, 84, 2, 171, 234, 164, 129, 227, 199, 85, 93, 120, 150, 163, 44, 45, 209, 67, 17, 177, 218, 247]
Decrypted: "Hello"
 */
```

Alternatively, you generate key pairs from a supplied 32-byte array:

```rust
    let mut my_sk: [u8; 32] = [0; 32];
    let my_sk = Asymmetric::to_secret(&mut my_sk);
    let my_pk = Asymmetric::public(&my_sk);
    
    let mut their_sk: [u8; 32] = [0; 32];
    let their_sk = Asymmetric::to_secret(&mut their_sk);
    let their_pk = Asymmetric::public(&their_sk);
```
