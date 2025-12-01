use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use advent_of_code_2025::solutions::*;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day01(c: &mut Criterion) {
    let input = include_str!("../inputs/day_01.txt").lines().collect_vec();
    let mut group = c.benchmark_group("day01");

    group.bench_function("part1", |b| {
        b.iter(|| day_01::solve_1(&input));
    });

    // group.bench_function("part2", |b| {
    //     b.iter(|| day_01::solve_2(&input));
    // });
}

criterion_group!(benches, day01);
criterion_main!(benches);
