use criterion::{criterion_group, criterion_main, Criterion};
use freivalds::freivalds_test;
use rand::Rng;

/// Generate a random n×n matrix with entries in [0, 10).
fn random_matrix(n: usize) -> Vec<Vec<u64>> {
    let mut rng = rand::rng();
    (0..n)
        .map(|_| (0..n).map(|_| rng.random_range(0..10)).collect())
        .collect()
}

fn bench_freivalds(c: &mut Criterion) {
    let n = 100;
    let a = random_matrix(n);
    let b = random_matrix(n);

    // Compute exact product c = a * b
    let c_mat = {
        let mut m = vec![vec![0u64; n]; n];
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    m[i][j] = m[i][j].wrapping_add(a[i][k].wrapping_mul(b[k][j]));
                }
            }
        }
        m
    };

    c.bench_function("freivalds_test 100×100 (5 iters)", |bench| {
        bench.iter(|| {
            // 5 iterations for high confidence
            assert!(freivalds_test(&a, &b, &c_mat, 5));
        })
    });
}

criterion_group!(freivalds_bench, bench_freivalds);
criterion_main!(freivalds_bench);
