use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2024::days::day04::{read_input, count_all_xmas, count_all_x_mas};

fn benchmark_day04(c: &mut Criterion) {
    let puzzle = read_input("data/day04.txt").expect("Failed to read and parse the input file");

    c.bench_function("count_all_xmas", |b| {
        b.iter(|| count_all_xmas(black_box(&puzzle)))
    });

    c.bench_function("count_all_x_mas", |b| {
        b.iter(|| count_all_x_mas(black_box(&puzzle)))
    });
}

criterion_group!(benches, benchmark_day04);
criterion_main!(benches);
