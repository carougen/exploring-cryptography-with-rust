//! Crate **`xgcd`** — elementary number-theory helpers.
//!
//! Two public functions:
//!
//! * [`gcd`]  – greatest common divisor via Euclid’s algorithm
//! * [`xgcd`] – extended version delivering the Bézout coefficients
//!   `x` and `y` such that `a·x + b·y = gcd(a,b)`.
//!
//! All arithmetic is done with `i128`, which already covers integers
//! up to ≈ 10³⁸.  Replace the type with `num_bigint::BigInt` if you need
//! arbitrary precision.
//!
//! # Quick example
//! ```
//! use xgcd::{gcd, xgcd};
//!
//! let (a, b) = (252_i128, 198);
//!
//! assert_eq!(gcd(a, b), 18);
//!
//! let (d, x, y) = xgcd(a, b);
//! assert_eq!(d, 18);
//! assert_eq!(a * x + b * y, d); // Bézout check
//! ```
//!
//! ```text
//! cargo test               # run unit tests
//! cargo run --example demo # small interactive demo
//! ```

/// Computes `gcd(a, b)` using the classical Euclidean algorithm.
///
/// Complexity: **O(log min(a,b))** divisions.
pub fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

/// Extended Euclidean algorithm.
///
/// Returns a triple **`(d, x, y)`** such that
/// `d = gcd(a,b)` **and** `a·x + b·y = d`.
///
/// Handy for modular inverses: if `gcd(a,n)=1`, then `x ≡ a⁻¹ (mod n)`.
pub fn xgcd(mut a: i128, mut b: i128) -> (i128, i128, i128) {
    let (mut x0, mut y0, mut x1, mut y1) = (1, 0, 0, 1);

    while b != 0 {
        let q = a / b;
        let r = a - q * b;
        a = b;
        b = r;

        let (next_x, next_y) = (x0 - q * x1, y0 - q * y1);
        x0 = x1;
        y0 = y1;
        x1 = next_x;
        y1 = next_y;
    }
    (a.abs(), x0, y0) // `a` might be negative; ensure gcd ≥ 0
}
