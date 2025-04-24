//! Crate **`xgcd`** — Elementary Number Theory Utilities
//!
//! This crate provides two core functions for integer arithmetic:
//!
//! - [`gcd`] — Computes the **Greatest Common Divisor (GCD)** using the classical Euclidean algorithm.
//! - [`xgcd`] — Computes the **Extended GCD**, returning Bézout coefficients `x` and `y` such that:
//!
//!   ```text
//!   a*x + b*y = gcd(a, b)
//!   ```
//!
//! All arithmetic is performed with `i128`, suitable for integers up to ~10^38.
//! For arbitrary-precision arithmetic, consider replacing `i128` with `num_bigint::BigInt`.
//!
//! ## Example Usage
//!
//! ```rust
//! use xgcd::{gcd, xgcd};
//!
//! let (a, b) = (252, 198);
//! assert_eq!(gcd(a, b), 18);
//!
//! let (d, x, y) = xgcd(a, b);
//! assert_eq!(d, 18);
//! assert_eq!(a * x + b * y, d); // Bézout identity
//! ```
//!
//! ## Development
//!
//! To run tests or examples:
//! ```text
//! cargo test -p xgcd               # Run unit tests
//! cargo run -p xgcd --example xgcd # Run example
//! ```

/// Computes the Greatest Common Divisor (GCD) of two integers `a` and `b`
/// using the classical Euclidean algorithm.
///
/// ## Complexity
/// - Time: **O(log(min(a, b)))** divisions
///
/// ## Example
/// ```
/// use xgcd::gcd;
/// assert_eq!(gcd(252, 198), 18);
/// ```
pub fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs() // Ensure non-negative result
}

/// Computes the Extended Euclidean Algorithm for integers `a` and `b`.
///
/// Returns a tuple `(d, x, y)` where:
/// - `d = gcd(a, b)`
/// - `x` and `y` satisfy the Bézout identity: `a * x + b * y = d`
///
/// ## Applications
/// Useful for computing **modular inverses**: if `gcd(a, n) = 1`, then `x ≡ a⁻¹ mod n`.
///
/// ## Example
/// ```
/// use xgcd::xgcd;
/// let (d, x, y) = xgcd(252, 198);
/// assert_eq!(d, 18);
/// assert_eq!(252 * x + 198 * y, d);
/// ```
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

    (a.abs(), x0, y0) // Return positive GCD and corresponding coefficients
}
