use aoc_2024::days::day01::{read_input, similarity_score, sorted_difference};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_day01(c: &mut Criterion) {
    let (first, second) =
        read_input("data/day01.txt").expect("Failed to read and parse the input file");

    c.bench_function("sorted_difference", |b| {
        b.iter(|| sorted_difference(black_box(&first), black_box(&second)))
    });

    c.bench_function("similarity_score", |b| {
        b.iter(|| similarity_score(black_box(&first), black_box(&second)))
    });
}

criterion_group!(benches, benchmark_day01);
criterion_main!(benches);
