use criterion::{criterion_group, criterion_main, Criterion};
use fingerprint::{codeword_lagrange, codeword_polynomial, evaluate_lagrange, evaluate_polynomial};

fn bench_evaluate_polynomial(c: &mut Criterion) {
    // 100-term polynomial over F₁₀₁, evaluated at x=7
    let message: Vec<u64> = (0..100).collect();
    let p = 101;
    let x = 7;
    c.bench_function("evaluate_polynomial (100 coeffs)", |bench| {
        bench.iter(|| {
            let _ = evaluate_polynomial(&message, p, x);
        })
    });
}

fn bench_evaluate_lagrange(c: &mut Criterion) {
    // 50-point Lagrange interpolation over F₁₀₁, at x=7
    let message: Vec<u64> = (1..=50).collect();
    let xs: Vec<u64> = (1..=50).collect();
    let p = 101;
    let x = 7;
    c.bench_function("evaluate_lagrange (50 points)", |bench| {
        bench.iter(|| {
            let _ = evaluate_lagrange(&message, &xs, p, x);
        })
    });
}

fn bench_codeword_polynomial(c: &mut Criterion) {
    // Generate RS codeword for p=127 from a 20-term message
    let message: Vec<u64> = (0..20).collect();
    let p = 127;
    c.bench_function("codeword_polynomial (p=127)", |bench| {
        bench.iter(|| {
            let _ = codeword_polynomial(&message, p);
        })
    });
}

fn bench_codeword_lagrange(c: &mut Criterion) {
    // Generate RS codeword via Lagrange for p=127 from 20 points
    let message: Vec<u64> = (0..20).collect();
    let xs: Vec<u64> = (0..20).collect();
    let p = 127;
    c.bench_function("codeword_lagrange (p=127)", |bench| {
        bench.iter(|| {
            let _ = codeword_lagrange(&message, &xs, p);
        })
    });
}

criterion_group!(
    fingerprint_benches,
    bench_evaluate_polynomial,
    bench_evaluate_lagrange,
    bench_codeword_polynomial,
    bench_codeword_lagrange
);
criterion_main!(fingerprint_benches);
