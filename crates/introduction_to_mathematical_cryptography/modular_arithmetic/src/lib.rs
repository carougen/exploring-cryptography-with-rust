//! Crate **`modular_arithmetic`** — Modular Arithmetic Utilities
//!
//! This crate provides core functions for modular arithmetic:
//!
//! - [`modular_inverse`] — Computes the **modular inverse** of an integer modulo `n`.
//! - [`euler_phi`] — Computes the **Euler's totient function** `φ(n)`.
//! - [`primitive_root`] — Finds a **primitive root** modulo a prime `p`.
//! - [`mod_pow`] — Computes **modular exponentiation** efficiently.
//!
//! All functions use `u64` integers, suitable for cryptographic-size primes.
//!
//! ## Example Usage
//!
//! ```rust
//! use modular_arithmetic::{modular_inverse, euler_phi, primitive_root, mod_pow};
//!
//! assert_eq!(modular_inverse(3, 11), Some(4));
//! assert_eq!(euler_phi(9), 6);
//! assert_eq!(primitive_root(7), Some(3));
//! assert_eq!(mod_pow(2, 5, 13), 6);
//! ```
//!
//! ## Development
//!
//! To run tests or examples:
//! ```text
//! cargo test -p modular_arithmetic
//! cargo run -p modular_arithmetic --example modular_examples
//! ```

/// Computes the modular inverse of `a` modulo `n`, if it exists.
/// Returns `Some(x)` such that `(a * x) % n == 1`, or `None` if no inverse.
///
/// ## Example
/// ```
/// use modular_arithmetic::modular_inverse;
/// assert_eq!(modular_inverse(3, 11), Some(4));
/// ```
pub fn modular_inverse(a: u64, n: u64) -> Option<u64> {
    let (mut t, mut new_t) = (0, 1);
    let (mut r, mut new_r) = (n as i64, a as i64);

    while new_r != 0 {
        let quotient = r / new_r;
        t -= quotient * new_t;
        r -= quotient * new_r;

        core::mem::swap(&mut t, &mut new_t);
        core::mem::swap(&mut r, &mut new_r);
    }

    if r > 1 {
        None
    } else {
        Some(((t + n as i64) % n as i64) as u64)
    }
}

/// Computes Euler's totient function `φ(n)`, the number of integers coprime to `n`.
///
/// ## Example
/// ```
/// use modular_arithmetic::euler_phi;
/// assert_eq!(euler_phi(9), 6);
/// ```
pub fn euler_phi(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }

    if n > 1 {
        result -= result / n;
    }

    result
}

/// Finds a primitive root modulo `p` (where `p` should be prime).
///
/// ## Example
/// ```
/// use modular_arithmetic::primitive_root;
/// assert_eq!(primitive_root(7), Some(3));
/// ```
pub fn primitive_root(p: u64) -> Option<u64> {
    if p == 2 {
        return Some(1);
    }

    let phi = p - 1;
    let mut factors = vec![];
    let mut n = phi;
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            factors.push(i);
            while n % i == 0 {
                n /= i;
            }
        }
        i += 1;
    }
    if n > 1 {
        factors.push(n);
    }

    'outer: for g in 2..p {
        for &factor in &factors {
            if mod_pow(g, phi / factor, p) == 1 {
                continue 'outer;
            }
        }
        return Some(g);
    }
    None
}

/// Computes `(base^exp) mod modulus` efficiently using **binary exponentiation**.
///
/// - Complexity: **O(log exp)** multiplications
///
/// ## Example
/// ```
/// use modular_arithmetic::mod_pow;
/// assert_eq!(mod_pow(2, 5, 13), 6); // 2^5 = 32, 32 mod 13 = 6
/// assert_eq!(mod_pow(7, 3, 10), 3); // 7^3 = 343, 343 mod 10 = 3
/// ```
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    result
}
