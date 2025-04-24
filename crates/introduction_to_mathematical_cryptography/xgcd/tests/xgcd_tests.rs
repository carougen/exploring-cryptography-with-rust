#[cfg(test)]
mod tests {
    use xgcd::{gcd, xgcd};

    #[test]
    fn small_examples() {
        assert_eq!(gcd(252, 198), 18);
        assert_eq!(gcd(17, 13), 1);
    }

    #[test]
    fn bezout_identity() {
        let a = 161_803; // arbitrary numbers
        let b = 50_000;
        let (d, x, y) = xgcd(a, b);
        assert_eq!(d, gcd(a, b));
        assert_eq!(a * x + b * y, d);
    }
}
