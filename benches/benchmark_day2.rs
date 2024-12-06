use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2024::days::day02::{read_input, count_safe_reports, count_safe_reports_dampener};

fn benchmark_day02(c: &mut Criterion) {
    let reports = read_input("data/day02.txt").expect("Failed to read and parse the input file");

    c.bench_function("count_safe_reports", |b| {
        b.iter(|| count_safe_reports(black_box(&reports)))
    });

    c.bench_function("count_safe_reports_dampener", |b| {
        b.iter(|| count_safe_reports_dampener(black_box(&reports)))
    });
}

criterion_group!(benches, benchmark_day02);
criterion_main!(benches);
