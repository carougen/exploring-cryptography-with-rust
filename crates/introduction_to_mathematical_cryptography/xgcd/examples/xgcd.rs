//! Example: gcd and xgcd.
//!
//! This standalone example demonstrates how to use the `gcd` and `xgcd` functions
//! from the `xgcd` crate to compute the greatest common divisor and Bézout coefficients.

use xgcd::{gcd, xgcd};

fn main() {
    // Define two sample integers.
    let a: i128 = 252;
    let b: i128 = 198;

    // Compute the greatest common divisor using Euclid’s algorithm.
    let d = gcd(a, b);

    // Compute the extended GCD: returns (d, x, y) such that a*x + b*y = d.
    let (d_ext, x, y) = xgcd(a, b);

    // Print the result of the standard GCD computation.
    println!("Gcd({}, {}) = {}", a, b, d);

    // Print the extended GCD result, including Bézout coefficients.
    println!("Xgcd({}, {}) -> d = {}, x = {}, y = {}", a, b, d_ext, x, y);

    // Verify the Bézout identity: a·x + b·y should equal gcd(a, b).
    println!(
        "Check : {}*{} + {}*{} = {} ({})",
        a,
        x,
        b,
        y,
        a * x + b * y,
        a * x + b * y == d
    );
}
