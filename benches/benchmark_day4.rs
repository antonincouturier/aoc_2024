use aoc_2024::days::day04::{count_all_x_mas, count_all_xmas, read_input};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
