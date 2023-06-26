#![allow(non_snake_case)]
#![allow(unused)]

use chacha20poly1305::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        ChaCha20Poly1305, Nonce,
};

pub fn decrypt(auth_key: String, cipher: String) -> String {
        todo!();
}

pub fn encrypt() {
        todo!();
}
