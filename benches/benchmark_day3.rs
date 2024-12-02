use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2024::days::day03::{read_input, compute_multiplications, compute_enabled_multiplications};

fn benchmark_day03(c: &mut Criterion) {
    let corrupted_memory = read_input("data/day03.txt");

    c.bench_function("compute_multiplications", |b| {
        b.iter(|| compute_multiplications(black_box(&corrupted_memory)))
    });

    c.bench_function("compute_enabled_multiplications", |b| {
        b.iter(|| compute_enabled_multiplications(black_box(&corrupted_memory)))
    });
}

criterion_group!(benches, benchmark_day03);
criterion_main!(benches);
