# bcl_oxide

Rust library that provides a simple interface for symmetric (i.e., secret-key) 
and asymmetric (i.e., public-key) encryption/decryption primitives. 

## Purpose
This library is a reimplementation of [bcl](https://github.com/nthparty/bcl), 
and is designed to provide the same functionalities for applications written in Rust.

## Package Installation

This package requires [sodiumoxide](https://github.com/sodiumoxide/sodiumoxide), which
must be installed manually. Once installed, edit the `path` variable in `Cargo.toml` to
point to the library's root directory.

## Examples

### Symmetric

```rust
extern crate bcl_oxide;
use bcl_oxide::symmetric::Symmetric;
use std::str;

fn main() {
    let k = Symmetric::secret();
    println!("Key: {:?}", k);

    let n = Symmetric::nonce();
    println!("Nonce: {:?}", n);

    let ct = Symmetric::encrypt(b"Hi", &n, &k);
    println!("Ciphertext: {:?}", ct);

    let ot = Symmetric::decrypt(&ct, &n, &k);
    println!("Opened: {:?}", ot);

    let uw = ot.unwrap();
    println!("Unwrapped: {:?}", uw);

    let pt = match str::from_utf8(&uw) {
        Ok(v) => v,
        _ => "err"
    };

    println!("Result: {}", pt);
}

/*
Output: 
Key: Key(****)
Nonce: Nonce([220, 105, 74, 167, 178, 31, 61, 209, 173, 35, 48, 79, 162, 220, 37, 144, 91, 110, 152, 85, 163, 212, 169, 232])
Ciphertext: [10, 233, 212, 132, 3, 225, 27, 91, 148, 223, 21, 240, 217, 103, 78, 162, 195, 100]
Opened: Ok([72, 105])
Unwrapped: [72, 105]
Result: Hi
 */
```

### Asymmetric

```rust
extern crate bcl_oxide;
use bcl_oxide::symmetric::Asymmetric;
use std::str;

fn main() {
    let (mypk, mysk) = Asymmetric::secret();
    println!("My keys: {:?} {:?}", mypk, mysk);

    let (otherpk, othersk) = Asymmetric::secret();
    println!("Their keys: {:?} {:?}", otherpk, othersk);

    let n = Asymmetric::nonce();
    println!("Nonce: {:?}", n);

    let ct = Asymmetric::encrypt(b"Hello", &n, &otherpk, &mysk);
    println!("Ciphertext: {:?}", ct);

    let ot = Asymmetric::decrypt(&ct, &n, &mypk, &othersk);
    println!("Opened: {:?}", ot);

    let uw = ot.unwrap();
    println!("Unwrapped: {:?}", uw);

    let pt = match str::from_utf8(&uw) {
        Ok(v) => v,
        _ => "err"
    };

    println!("Result: {}", pt);
}

/*
Output:
My keys: PublicKey([104, 216, 185, 202, 170, 145, 215, 121, 207, 25, 136, 218, 140, 78, 176, 48, 41, 127, 57, 225, 248, 125, 165, 9, 200, 11, 30, 53, 53, 112, 214, 15]) SecretKey(****)
Their keys: PublicKey([117, 123, 62, 159, 164, 223, 96, 228, 36, 254, 18, 67, 213, 200, 246, 127, 61, 125, 56, 92, 186, 123, 135, 8, 36, 19, 18, 165, 16, 248, 58, 21]) SecretKey(****)
Nonce: Nonce([118, 230, 44, 13, 23, 41, 160, 45, 110, 13, 179, 171, 83, 72, 144, 191, 241, 232, 114, 155, 247, 140, 37, 125])
Ciphertext: [34, 182, 223, 162, 14, 140, 174, 42, 94, 22, 196, 74, 233, 170, 44, 232, 146, 244, 107, 87, 185]
Opened: Ok([72, 101, 108, 108, 111])
Unwrapped: [72, 101, 108, 108, 111]
Result: Hello
 */
```