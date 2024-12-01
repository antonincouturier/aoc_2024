use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2024::days::day01::{read_input, sorted_difference, similarity_score};

fn benchmark_day01(c: &mut Criterion) {
    let (first, second) = read_input("data/day01.txt");
    // println!("Input sizes: {} and {}", first.len(), second.len()); 

    c.bench_function("sorted_difference", |b| {
        b.iter(|| sorted_difference(black_box(&first), black_box(&second)))
    });

    c.bench_function("similarity_score", |b| {
        b.iter(|| similarity_score(black_box(&first), black_box(&second)))
    });
}

criterion_group!(benches, benchmark_day01);
criterion_main!(benches);
