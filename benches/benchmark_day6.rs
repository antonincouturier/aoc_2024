use aoc_2024::days::day06::{find_all_loops_parallel, guard_patrol_count, read_input};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_day06(c: &mut Criterion) {
    let (start, map) = read_input("data/day06.txt").expect("Failed to read input");

    c.bench_function("guard_patrol_count", |b| {
        b.iter(|| guard_patrol_count(black_box(&start), black_box(&map)))
    });

    c.bench_function("find_all_loops_parallel", |b| {
        b.iter(|| find_all_loops_parallel(black_box(&start), black_box(&map)))
    });
}

criterion_group!(benches, benchmark_day06);
criterion_main!(benches);
