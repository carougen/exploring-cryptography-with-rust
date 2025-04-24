use freivalds::freivalds_test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freivalds_correct() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let b = vec![vec![2, 0], vec![1, 2]];
        // c = a * b = [[4,4],[10,8]]
        let c = vec![vec![4, 4], vec![10, 8]];
        assert!(freivalds_test(&a, &b, &c, 5));
    }

    #[test]
    fn test_freivalds_incorrect() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let b = vec![vec![2, 0], vec![1, 2]];
        let c_wrong = vec![vec![9, 9], vec![9, 9]];
        assert!(!freivalds_test(&a, &b, &c_wrong, 5));
    }
}
