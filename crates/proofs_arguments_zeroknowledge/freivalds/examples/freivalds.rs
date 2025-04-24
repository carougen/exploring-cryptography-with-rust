// crates/freivalds-matrix/examples/freivalds.rs

use freivalds::freivalds_test;

fn main() {
    // define matrices a, b
    let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 10]];
    let b = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];

    // c = a * b should equal a
    let c = a.clone();

    // number of random checks
    let iterations = 10;

    // run Freivalds' test on correct product
    let ok = freivalds_test(&a, &b, &c, iterations);
    println!("A = {a:?}\nB = {b:?}\nC = {c:?}");
    println!("Test a * b == c: {}\n", ok);

    // introduce an error in c
    let mut c_bad = c.clone();
    c_bad[2][1] = 0; // corrupt one entry
    println!("A = {a:?}\nB = {b:?}\nC_bad = {c_bad:?}");
    let ok_bad = freivalds_test(&a, &b, &c_bad, iterations);
    println!("Test a * b == c_bad: {}", ok_bad);
}
