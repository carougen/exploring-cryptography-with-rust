//! Example: using core modular arithmetic utilities
//!
//! Demonstrates usage of:
//! - modular inverse computation
//! - Euler's totient function
//! - primitive root finding
//! - fast modular exponentiation

use modular_arithmetic::*;

fn main() {
    // === Modular Inverse Examples ===
    println!("== Modular Inverse Examples ==");

    let cases = vec![(3, 11), (10, 17), (7, 26)];

    for (a, n) in cases {
        match modular_inverse(a, n) {
            Some(inv) => println!("The modular inverse of {} mod {} is: {}", a, n, inv),
            None => println!("No modular inverse exists for {} mod {}", a, n),
        }
    }
    // Expected:
    // - 3 mod 11 => inverse 4 (because 3×4 ≡ 1 mod 11)
    // - 10 mod 17 => inverse 12
    // - 7 mod 26 => no inverse (not coprime)

    // === Euler's Totient Function Examples ===
    println!("\n== Euler's Totient Function Examples ==");

    let numbers = vec![9, 10, 12];
    for n in numbers {
        let phi = euler_phi(n);
        println!("Euler's phi function of {} is: {}", n, phi);
    }
    // Expected:
    // - φ(9) = 6 (numbers coprime to 9: 1,2,4,5,7,8)
    // - φ(10) = 4 (1,3,7,9)
    // - φ(12) = 4 (1,5,7,11)

    // === Primitive Root Examples ===
    println!("\n== Primitive Root Examples ==");

    let primes = vec![7, 11, 17];
    for p in primes {
        match primitive_root(p) {
            Some(root) => println!("A primitive root modulo {} is: {}", p, root),
            None => println!("No primitive root found for {}", p),
        }
    }
    // Expected:
    // - 3 is a primitive root mod 7
    // - Some primitive root exists for 11 and 17

    // === Modular Exponentiation Examples ===
    println!("\n== Modular Exponentiation Examples ==");

    let exps = vec![
        (2, 5, 13), // 2^5 mod 13 = 6
        (7, 3, 10), // 7^3 mod 10 = 3
        (5, 0, 17), // 5^0 mod 17 = 1
    ];

    for (base, exp, modulus) in exps {
        let result = mod_pow(base, exp, modulus);
        println!("{}^{} mod {} = {}", base, exp, modulus, result);
    }
    // Expected:
    // - 2^5 mod 13 = 6
    // - 7^3 mod 10 = 3
    // - 5^0 mod 17 = 1 (any number to the power 0 modulo n is 1)
}
