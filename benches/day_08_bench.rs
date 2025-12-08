use advent_of_code_2025::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day08(c: &mut Criterion) {
    let input = include_str!("../inputs/day_08.txt").lines().collect_vec();
    let mut group = c.benchmark_group("day08");

    group.bench_function("part1", |b| {
        b.iter(|| day_08::solve_1(&input, 1_000));
    });

    group.bench_function("part2", |b| {
        b.iter(|| day_08::solve_2(&input));
    });
}

criterion_group!(benches, day08);
criterion_main!(benches);
