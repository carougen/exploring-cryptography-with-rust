//! Example: gcd and xgcd.
use xgcd::{gcd, xgcd};

fn main() {
    // Pick any numbers:
    let a: i128 = 252;
    let b: i128 = 198;

    // Plain GCD
    let d = gcd(a, b);

    // Extended GCD (gives Bézout coefficients)
    let (d_ext, x, y) = xgcd(a, b);

    println!("Gcd({}, {}) = {}", a, b, d);
    println!("Xgcd({}, {}) -> d = {}, x = {}, y = {}", a, b, d_ext, x, y);
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
