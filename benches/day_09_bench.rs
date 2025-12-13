use advent_of_code_2025::solutions::*;
use criterion::{Criterion, criterion_group, criterion_main};
use itertools::Itertools;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day09(c: &mut Criterion) {
    let input = include_str!("../inputs/day_09.txt").lines().collect_vec();
    let mut group = c.benchmark_group("day09");

    group.bench_function("part1", |b| {
        b.iter(|| day_09::solve_1(&input));
    });

    group.bench_function("part2", |b| {
        b.iter(|| day_09::solve_2(&input));
    });
}

criterion_group!(benches, day09);
criterion_main!(benches);
