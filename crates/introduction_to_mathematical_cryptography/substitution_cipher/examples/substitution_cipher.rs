//! Example: simple substitution cipher.
//!
//! Demonstrates encrypting and decrypting.

use substitution_cipher::{decrypt_substitution, encrypt_substitution};

fn main() {
    let key = "CISQVNFOWAXMTGUHPBKLREYDZJ";
    let plaintext = "four score and seven years ago";
    println!("Plaintext: \"{}\"", plaintext);

    let cipher = encrypt_substitution(plaintext, key);
    println!("Encrypted: \"{}\"", cipher);
    // Ciphertext: nurb ksubv cgq kvevg zvcbk cfu

    let recovered = decrypt_substitution(&cipher, key);
    println!("Decrypted: \"{}\"", recovered);
    // Plaintext: four score and seven years ago
}
