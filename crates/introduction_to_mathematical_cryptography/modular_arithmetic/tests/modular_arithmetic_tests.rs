#[cfg(test)]
mod tests {
    use modular_arithmetic::{euler_phi, mod_pow, modular_inverse, primitive_root};

    #[test]
    fn test_modular_inverse() {
        assert_eq!(modular_inverse(3, 11), Some(4));
        assert_eq!(modular_inverse(10, 17), Some(12));
        assert_eq!(modular_inverse(12, 8), None);
    }

    #[test]
    fn test_euler_phi() {
        assert_eq!(euler_phi(1), 1);
        assert_eq!(euler_phi(2), 1);
        assert_eq!(euler_phi(9), 6);
        assert_eq!(euler_phi(10), 4);
    }

    #[test]
    fn test_primitive_root() {
        assert_eq!(primitive_root(2), Some(1));
        assert_eq!(primitive_root(7), Some(3)); // 3 is a generator mod 7
        assert_eq!(primitive_root(11).is_some(), true);
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 5, 13), 6);
        assert_eq!(mod_pow(7, 3, 10), 3);
        assert_eq!(mod_pow(5, 0, 17), 1); // any number^0 mod n == 1
        assert_eq!(mod_pow(0, 5, 7), 0); // 0^k mod n == 0
    }
}
