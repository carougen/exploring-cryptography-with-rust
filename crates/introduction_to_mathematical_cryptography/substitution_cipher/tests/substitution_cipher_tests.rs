#[cfg(test)]
mod tests {
    use substitution_cipher::{decrypt_substitution, encrypt_substitution};

    const BOOK_KEY: &str = "CISQVNFOWAXMTGUHPBKLREYDZJ";

    #[test]
    fn encrypt_book_example() {
        let pt = "four score and seven years ago";
        let ct = encrypt_substitution(pt, BOOK_KEY);
        assert_eq!(ct, "NURB KSUBV CGQ KVEVG ZVCBK CFU");
    }

    #[test]
    fn decrypt_book_example() {
        let ct = "NURB KSUBV CGQ KVEVG ZVCBK CFU";
        let pt = decrypt_substitution(ct, BOOK_KEY);
        assert_eq!(pt, "four score and seven years ago");
    }

    #[test]
    fn non_alpha_preserved() {
        let key = "ZYXWVUTSRQPONMLKJIHGFEDCBA";
        let text = "hello, world!";
        // with reverse alphabet, 'h'→'S', 'e'→'V', ...
        let ct = encrypt_substitution(text, key);
        assert!(ct.contains(","));
        let rt = decrypt_substitution(&ct, key);
        assert_eq!(rt, "hello, world!");
    }
}
