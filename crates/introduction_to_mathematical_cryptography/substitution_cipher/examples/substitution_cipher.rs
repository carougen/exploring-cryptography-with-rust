//! Example: simple substitution cipher.
//!
//! Demonstrates how to encrypt and decrypt a message using a
//! monoalphabetic substitution cipher defined by a 26-letter key.

use substitution_cipher::{decrypt_substitution, encrypt_substitution};

fn main() {
    // Define a substitution key: must be 26 uppercase ASCII letters (A–Z)
    // Maps: 'a' → 'C', 'b' → 'I', ..., 'z' → 'J'
    let key = "CISQVNFOWAXMTGUHPBKLREYDZJ";

    // Plaintext input (any ASCII string)
    let plaintext = "four score and seven years ago";
    println!("Plaintext : \"{}\"", plaintext);

    // Encrypt the plaintext (result will be all-uppercase)
    let cipher = encrypt_substitution(plaintext, key);
    println!("Encrypted : \"{}\"", cipher);
    // Output should be: NURB KSUBV CGQ KVEVG ZVCBK CFU

    // Decrypt the ciphertext (returns lowercase)
    let recovered = decrypt_substitution(&cipher, key);
    println!("Decrypted : \"{}\"", recovered);
    // Output should match the original: four score and seven years ago
}
