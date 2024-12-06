use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2024::days::day05::{read_input, middle_page_sum, reordered_middle_page_sum};

fn benchmark_day05(c: &mut Criterion) {
    let (rules, updates) = read_input("data/day05.txt").expect("Failed to read input");

    c.bench_function("middle_page_sum", |b| {
        b.iter(|| middle_page_sum(black_box(&updates), black_box(&rules)))
    });

    c.bench_function("reordered_middle_page_sum", |b| {
        b.iter(|| reordered_middle_page_sum(black_box(&updates), black_box(&rules)))
    });
}

criterion_group!(benches, benchmark_day05);
criterion_main!(benches);

