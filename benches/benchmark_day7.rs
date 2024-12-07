use aoc_2024::days::day07::{read_input, total_calibration, total_calibration_concat};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_day07(c: &mut Criterion) {
    let calibration_data = read_input("data/day07.txt").expect("Failed to read input");

    c.bench_function("total_calibration", |b| {
        b.iter(|| total_calibration(black_box(&calibration_data)))
    });

    c.bench_function("total_calibration_concat", |b| {
        b.iter(|| total_calibration_concat(black_box(&calibration_data)))
    });
}

criterion_group!(benches, benchmark_day07);
criterion_main!(benches);
