//! Crate **`substitution_cipher`** — Simple Monoalphabetic Substitution Cipher
//!
//! This crate provides basic functions to **encrypt** and **decrypt** ASCII text
//! using a fixed-key monoalphabetic substitution cipher.
//!
//! Each letter `a`–`z` is mapped to a letter from a 26-character key (e.g., `CISQVNFOWAXMTGUHPBKLREYDZJ`).
//! Non-alphabetic characters (spaces, punctuation, digits) are left **unchanged**.
//!
//! All encrypted output is **uppercase**, and decryption returns **lowercase**.
//!
//! ## Example Usage
//!
//! ```rust
//! use substitution_cipher::{encrypt_substitution, decrypt_substitution};
//!
//! let key = "CISQVNFOWAXMTGUHPBKLREYDZJ";  // key maps: a → C, b → I, ..., z → J
//! let plaintext = "four score and seven years ago";
//!
//! let cipher = encrypt_substitution(plaintext, key);
//! assert_eq!(&cipher, "NURB KSUBV CGQ KVEVG ZVCBK CFU");
//!
//! let recovered = decrypt_substitution(&cipher, key);
//! assert_eq!(&recovered, plaintext);
//! ```

/// Encrypts the given `input` using a 26-character substitution `key`.
///
/// # Arguments
/// - `input`: ASCII string to encrypt (letters and symbols).
/// - `key`: A 26-letter uppercase ASCII string (`A–Z`) where:
///   - `key[0]` is the substitution for `'a'`
///   - `key[1]` is the substitution for `'b'`
///   - ...
///   - `key[25]` is for `'z'`
///
/// # Behavior
/// - Input is case-insensitive.
/// - Output is always **uppercase**.
/// - Non-alphabetic characters (e.g. space, digits, punctuation) are not modified.
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

/// Decrypts a string previously encrypted with the same substitution `key`.
///
/// # Arguments
/// - `input`: Encrypted message using uppercase ASCII letters.
/// - `key`: The same 26-letter key used during encryption.
///
/// # Returns
/// - The original **lowercase** plaintext message.
///
/// # Panics
/// - If the `key` contains non-uppercase letters.
/// - If the key is not exactly 26 characters.
///
/// # Example
/// ```
/// use substitution_cipher::decrypt_substitution;
/// let key = "CISQVNFOWAXMTGUHPBKLREYDZJ";
/// let msg = "NURB KSUBV CGQ KVEVG ZVCBK CFU";
/// assert_eq!(decrypt_substitution(msg, key), "four score and seven years ago");
/// ```
pub fn decrypt_substitution(input: &str, key: &str) -> String {
    let mut inv = [b'?'; 26];
    for (i, &b) in key.as_bytes().iter().enumerate() {
        let pos = if b.is_ascii_uppercase() {
            (b - b'A') as usize
        } else {
            panic!("key must contain only uppercase A–Z; found {}", b as char)
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
