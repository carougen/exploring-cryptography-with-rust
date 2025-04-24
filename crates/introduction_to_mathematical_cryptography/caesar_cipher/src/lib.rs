//! Crate **`caesar_cipher`** — Caesar Cipher with Configurable Rotation
//!
//! This crate provides functions for **encrypting** and **decrypting** ASCII text
//! using the classic Caesar cipher. It shifts each letter by a fixed number of positions
//! in the alphabet. The shift is cyclic, and non-alphabetic characters are preserved.
//!
//! ## Behavior
//! - Supports both **uppercase** and **lowercase** letters.
//! - Non-letter characters (punctuation, digits, spaces) are **not changed**.
//! - Shift must be an integer from `0` to `25`.
//!
//! ## Example Usage
//!
//! ```rust
//! use caesar_cipher::{encrypt_caesar, decrypt_caesar};
//!
//! let plain = "Hello, World!";
//! let cipher = encrypt_caesar(plain, 3);
//! assert_eq!(&cipher, "Khoor, Zruog!");
//!
//! let recovered = decrypt_caesar(&cipher, 3);
//! assert_eq!(&recovered, plain);
//! ```

/// Encrypts a message using the Caesar cipher with a fixed `shift`.
///
/// # Arguments
/// - `input`: the plaintext string to encrypt.
/// - `shift`: number of positions to shift each letter (0–25).
///
/// # Returns
/// A new `String` with each letter shifted by `shift` positions.
/// Wraps around the alphabet (e.g. `'z' + 1 → 'a'`, `'Z' + 2 → 'B'`).
///
/// # Example
/// ```
/// use caesar_cipher::encrypt_caesar;
/// assert_eq!(encrypt_caesar("abc XYZ", 2), "cde ZAB");
/// ```
pub fn encrypt_caesar(input: &str, shift: u8) -> String {
    input.chars().map(|c| shift_char(c, shift as i8)).collect()
}

/// Decrypts a message that was encrypted using the Caesar cipher.
///
/// # Arguments
/// - `input`: the ciphertext to decrypt.
/// - `shift`: original shift used during encryption (0–25).
///
/// # Returns
/// A new `String` containing the decrypted plaintext.
///
/// Decryption is done by inverting the shift: `26 - (shift % 26)`.
///
/// # Example
/// ```
/// use caesar_cipher::decrypt_caesar;
/// assert_eq!(decrypt_caesar("Khoor", 3), "Hello");
/// ```
pub fn decrypt_caesar(input: &str, shift: u8) -> String {
    encrypt_caesar(input, 26 - (shift % 26))
}

/// Internal helper: shifts a single character by `shift` positions.
///
/// - Only affects ASCII letters (a–z, A–Z)
/// - Preserves character case
/// - Uses modular arithmetic for wraparound
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
        c // Non-letter characters are not changed
    }
}
