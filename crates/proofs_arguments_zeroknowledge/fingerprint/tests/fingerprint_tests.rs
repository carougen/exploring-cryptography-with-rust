use fingerprint::{codeword_lagrange, codeword_polynomial, evaluate_polynomial};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_polynomial_known() {
        let p = 11;
        // message a
        let a = vec![2, 1, 1];
        // f(x) = 2 + x + x^2
        assert_eq!(evaluate_polynomial(&a, p, 0), 2);
        assert_eq!(evaluate_polynomial(&a, p, 1), 4 % p);
        assert_eq!(evaluate_polynomial(&a, p, 2), 8 % p);
    }

    #[test]
    fn test_codeword_polynomial_full() {
        let p = 11;
        let a = vec![2, 1, 1];
        let expected_a = vec![2, 4, 8, 3, 0, 10, 0, 3, 8, 4, 2];
        let cw_a = codeword_polynomial(&a, p);
        assert_eq!(cw_a, expected_a);

        let b = vec![2, 1, 0];
        let expected_b = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 1];
        let cw_b = codeword_polynomial(&b, p);
        assert_eq!(cw_b, expected_b);
    }

    #[test]
    fn test_codeword_lagrange_same() {
        let p = 11;
        let xs = vec![0, 1, 2];
        let a = vec![2, 1, 1];
        let expected_a = vec![2, 1, 1, 2, 4, 7, 0, 5, 0, 7, 4];
        let cw_a = codeword_lagrange(&a, &xs, p);
        assert_eq!(cw_a, expected_a);

        let b = vec![2, 1, 0];
        let expected_b = vec![2, 1, 0, 10, 9, 8, 7, 6, 5, 4, 3];
        let cw_b = codeword_lagrange(&b, &xs, p);
        assert_eq!(cw_b, expected_b);
    }
}
