//! Crate `freivalds_matrix`: Freivalds' algorithm for verifying matrix multiplication.
//!
//! This library provides a probabilistic function to check whether a * b = c
//! in O(k * n^2) time, where k is the number of iterations (confidence parameter).

use rand::{rng, Rng};

/// Multiplies a matrix by a vector.
///
/// Each element of the resulting vector is computed as the dot product of the
/// corresponding row of the matrix `m` with the vector `v`.
///
/// # Arguments
/// * `m` - a slice of rows, each row being a `Vec<u64>` of length n
/// * `v` - a vector of length n
///
/// # Returns
/// A new `Vec<u64>` of length n, where the i-th entry is:
/// `sum_{j=0..n} (m[i][j] * v[j]) mod 2^64`.
fn mul_mat_vec(m: &[Vec<u64>], v: &[u64]) -> Vec<u64> {
    m.iter()
        .map(|row| {
            row.iter().zip(v).fold(0u64, |sum, (&m_ij, &v_j)| {
                sum.wrapping_add(m_ij.wrapping_mul(v_j))
            })
        })
        .collect()
}

/// Perform Freivalds' algorithm to verify a * b == c.
///
/// # Arguments
/// * `a`, `b`, `c` - square matrices of size n x n (`Vec<Vec<u64>>`)
/// * `iterations` - number of random checks (higher -> lower error probability)
///
/// # Returns
/// * `true` if the test passes all iterations (likely a*b == c)
/// * `false` if a mismatch is detected (guaranteed if a*b != c)
pub fn freivalds_test(a: &[Vec<u64>], b: &[Vec<u64>], c: &[Vec<u64>], iterations: u32) -> bool {
    let n = a.len();
    let mut rng = rng();
    let mut r = vec![0u64; n];

    for _ in 0..iterations {
        // generate random vector r in {0,1}^n
        for elem in &mut r {
            *elem = rng.random_range(0..2);
        }

        // compute br = b * r
        let br = mul_mat_vec(b, &r);

        // compute abr = a * br
        let abr = mul_mat_vec(a, &br);

        // compute cr = c * r
        let cr = mul_mat_vec(c, &r);

        // if vectors differ, the product is incorrect
        if abr != cr {
            return false;
        }
    }

    // all tests passed, likely a*b == c
    true
}
