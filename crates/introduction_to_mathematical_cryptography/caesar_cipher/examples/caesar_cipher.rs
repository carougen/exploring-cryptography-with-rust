//! Example: decrypt the Caesar-cipher snippet from the book.
//!
//! Demonstrates decryption of a Caesar cipher with known shift,
//! and re-encryption to verify correctness.

use caesar_cipher::{decrypt_caesar, encrypt_caesar};

fn main() {
    // Ciphertext as found in the book (encrypted with Caesar cipher, shift = 5)
    let cipher =
        "j s j r d k f q q n s l g f h p g w j f p y m w t z l m n r r n s j s y q z h n z x";
    println!("Cipher     : \"{}\"", cipher);

    // Decrypt using the same Caesar shift of 5
    let plain = decrypt_caesar(cipher, 5);
    println!("Decrypted  : \"{}\"", plain);
    // Expected: "e n e m y f a l l i n g b a c k b r e a k t h r o u g h i m m i n e n t l u c i u s"

    // Re-encrypt the plaintext to confirm it matches the original ciphertext
    let re = encrypt_caesar(&plain, 5);
    println!("Re-encrypted: \"{}\"", re);
    // Should match the original cipher exactly
}
