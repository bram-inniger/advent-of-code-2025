use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2025::solutions::*;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day02(c: &mut Criterion) {
    let input = include_str!("../inputs/day_02.txt").trim();
    let mut group = c.benchmark_group("day01");

    group.bench_function("part1", |b| {
        b.iter(|| day_02::solve_1(input));
    });

    group.bench_function("part2", |b| {
        b.iter(|| day_02::solve_2(input));
    });
}

criterion_group!(benches, day02);
criterion_main!(benches);
