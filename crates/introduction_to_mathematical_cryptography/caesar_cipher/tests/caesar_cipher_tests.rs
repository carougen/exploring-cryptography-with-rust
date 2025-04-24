#[cfg(test)]
mod tests {
    use caesar_cipher::{decrypt_caesar, encrypt_caesar};

    #[test]
    fn basic_encrypt() {
        assert_eq!(encrypt_caesar("abcXYZ", 3), "defABC");
        assert_eq!(encrypt_caesar("Rust 1.0!", 13), "Ehfg 1.0!");
    }

    #[test]
    fn basic_decrypt() {
        let text = "Attack at dawn!";
        let c = encrypt_caesar(text, 7);
        assert_eq!(decrypt_caesar(&c, 7), text);
    }

    #[test]
    fn wrap_around() {
        assert_eq!(encrypt_caesar("xyz", 4), "bcd");
        assert_eq!(decrypt_caesar("BCD", 4), "XYZ");
    }

    #[test]
    fn non_alpha_unchanged() {
        assert_eq!(encrypt_caesar("123!?", 5), "123!?");
    }
}
