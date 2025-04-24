//! Crate `caesar_cipher`: Caesar cipher with configurable rotation.
//!
//! This library provides simple Caesar-cipher encryption and decryption
//! of ASCII text, with a user-specified shift (0..25). Non-alphabetic
//! characters are left unchanged.
//!
//! # Examples
//!
//! ```rust
//! use caesar_cipher::{encrypt_caesar, decrypt_caesar};
//!
//! let plain = "Hello, World!";
//! let cipher = encrypt_caesar(plain, 3);
//! assert_eq!(&cipher, "Khoor, Zruog!");
//! assert_eq!(&decrypt_caesar(&cipher, 3), plain);
//! ```

/// Encrypts `input` by shifting each ASCII letter by `shift` positions.
///
/// Letters wrap around the alphabet:
/// `'z' + 1 == 'a'`, `'Z' + 2 == 'B'`.
/// Other characters (digits, punctuation, whitespace) are passed through.
///
/// # Arguments
///
/// * `input` — the plaintext to encrypt
/// * `shift` — number of positions to rotate (0..=25)
///
/// # Returns
///
/// A new `String` containing the ciphertext.
pub fn encrypt_caesar(input: &str, shift: u8) -> String {
    input.chars().map(|c| shift_char(c, shift as i8)).collect()
}

/// Decrypts `input` that was encrypted with the same `shift`.
///
/// Decryption is equivalent to encrypting with `26 − (shift % 26)`.
///
/// # Arguments
///
/// * `input` — the ciphertext to decrypt
/// * `shift` — the original shift used for encryption (0..=25)
///
/// # Returns
///
/// A new `String` containing the recovered plaintext.
pub fn decrypt_caesar(input: &str, shift: u8) -> String {
    // reverse shift by 26 - (shift mod 26)
    encrypt_caesar(input, 26 - (shift % 26))
}

/// Shift a single `char` by `shift` positions in the ASCII alphabet.
/// If `c` is in 'a'..='z' or 'A'..='Z', it is rotated; otherwise returns `c`.
fn shift_char(c: char, shift: i8) -> char {
    if c.is_ascii_lowercase() {
        let base = b'a';
        let idx = c as u8 - base;
        let rotated = (idx as i8 + shift).rem_euclid(26) as u8 + base;
        rotated as char
    } else if c.is_ascii_uppercase() {
        let base = b'A';
        let idx = c as u8 - base;
        let rotated = (idx as i8 + shift).rem_euclid(26) as u8 + base;
        rotated as char
    } else {
        c
    }
}
