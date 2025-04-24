//! Crate `substitution_cipher`: simple monoalphabetic substitution cipher.
//!
//! This library provides functions to encrypt and decrypt ASCII text
//! using an arbitrary 26-letter key mapping `a..z → key[0]..key[25]`.
//! Non-alphabetic characters are left unchanged.
//!
//! # Example
//!
//! ```rust
//! use substitution_cipher::{encrypt_substitution, decrypt_substitution};
//!
//! // key: maps 'a'→'C', 'b'→'I', ..., 'z'→'J'
//! let key = "CISQVNFOWAXMTGUHPBKLREYDZJ";
//! let plaintext = "four score and seven years ago";
//! let cipher   = encrypt_substitution(plaintext, key);
//! assert_eq!(&cipher, "NURB KSUBV CGQ KVEVG ZVCBK CFU");
//!
//! let recovered = decrypt_substitution(&cipher, key);
//! assert_eq!(&recovered, plaintext);
//! ```

/// Encrypts `input` by substituting each ASCII letter according to `key`.
///
/// * `key` must be a 26-byte ASCII string: key[0] is cipher for 'a', key[1] for 'b', …
/// * Input letters are case-insensitive and always mapped to **uppercase**.
/// * Non-alphabetic chars are copied unchanged.
pub fn encrypt_substitution(input: &str, key: &str) -> String {
    let key_bytes = key.as_bytes();
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let idx = (c.to_ascii_lowercase() as u8 - b'a') as usize;
                key_bytes[idx] as char
            } else {
                c
            }
        })
        .collect()
}

/// Decrypts `input` that was encrypted with the same `key`.
///
/// Builds the inverse mapping and returns a **lowercase** plaintext.
pub fn decrypt_substitution(input: &str, key: &str) -> String {
    // build inverse: cipher byte → plaintext byte
    let mut inv = [b'?'; 26];
    for (i, &b) in key.as_bytes().iter().enumerate() {
        let pos = if b.is_ascii_uppercase() {
            (b - b'A') as usize
        } else {
            panic!("key must be A–Z; found {}", b as char)
        };
        inv[pos] = b'a' + i as u8;
    }
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let pos = (c.to_ascii_uppercase() as u8 - b'A') as usize;
                inv[pos] as char
            } else {
                c
            }
        })
        .collect()
}
