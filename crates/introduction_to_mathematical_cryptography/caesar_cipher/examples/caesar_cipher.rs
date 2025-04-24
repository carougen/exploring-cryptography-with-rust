//! Example: decrypt the Caesar‐cipher snippet from the book

use caesar_cipher::{decrypt_caesar, encrypt_caesar};

fn main() {
    // Ciphertext from the book (shift = +5)
    let cipher =
        "j s j r d k f q q n s l g f h p g w j f p y m w t z l m n r r n s j s y q z h n z x";
    println!("Cipher: \"{}\"", cipher);

    // Decrypt with the same shift
    let plain = decrypt_caesar(cipher, 5);
    println!("Decrypted: \"{}\"", plain);
    // Decrypted: "e n e m y f a l l i n g b a c k b r e a k t h r o u g h i m m i n e n t l u c i u s"

    // Re-encrypt to verify round-trip
    let re = encrypt_caesar(&plain, 5);
    println!("Re-encrypted: \"{}\"", re);
    // Re-encrypted: "j s j r d k f q q n s l g f h p g w j f p y m w t z l m n r r n s j s y q z h n z x"
}
