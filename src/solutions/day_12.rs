use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::sync::LazyLock;

pub fn solve_1(summary: &str, simple: bool) -> usize {
    let parts = summary.split("\n\n").collect_vec();
    let _shapes = parts[0..parts.len() - 1]
        .iter()
        .map(|shape| Shape::new(shape))
        .map(|shape| (shape.idx, shape))
        .collect::<FxHashMap<_, _>>();
    let regions = parts[parts.len() - 1]
        .split('\n')
        .map(|region| Region::new(region))
        .collect_vec();

    if !simple {
        2 // TODO implement proper backtracking shape fitting
    } else {
        regions
            .iter()
            .filter(|region| {
                let shapes_max = (region.width / 3) * (region.height / 3);
                let shapes_required = region.shape_counts.iter().sum::<u32>();

                shapes_max >= shapes_required
            })
            .count()
    }
}

pub fn solve_2() {
    // Decorate the North Pole
}

#[derive(Debug, Clone)]
struct Shape {
    idx: usize,
    _grid: [[bool; 3]; 3],
}

#[derive(Debug, Clone)]
struct Region {
    width: u32,
    height: u32,
    shape_counts: Vec<u32>,
}

impl Shape {
    fn new(shape: &str) -> Self {
        static IDX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?<idx>\d+):$").unwrap());

        let lines = shape.lines().collect_vec();

        let idx = IDX_RE.captures(lines[0]).unwrap()["idx"].parse().unwrap();
        let _grid = lines
            .iter()
            .skip(1)
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => unreachable!(),
                    })
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect_vec()
            .try_into()
            .unwrap();

        Self { idx, _grid }
    }
}

impl Region {
    fn new(region: &str) -> Self {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^(?<width>\d+)x(?<height>\d+): (?<shape_counts>.*)$").unwrap()
        });

        let caps = RE.captures(region).unwrap();

        let width = caps["width"].parse().unwrap();
        let height = caps["height"].parse().unwrap();
        let shape_counts = caps["shape_counts"]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            width,
            height,
            shape_counts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_12_part_01_sample() {
        let sample = "\
            0:\n\
            ###\n\
            ##.\n\
            ##.\n\
            \n\
            1:\n\
            ###\n\
            ##.\n\
            .##\n\
            \n\
            2:\n\
            .##\n\
            ###\n\
            ##.\n\
            \n\
            3:\n\
            ##.\n\
            ###\n\
            ##.\n\
            \n\
            4:\n\
            ###\n\
            #..\n\
            ###\n\
            \n\
            5:\n\
            ###\n\
            .#.\n\
            ###\n\
            \n\
            4x4: 0 0 0 0 2 0\n\
            12x5: 1 0 1 0 2 2\n\
            12x5: 1 0 1 0 3 2\
        ";

        assert_eq!(2, solve_1(sample, false));
    }

    #[test]
    fn day_12_part_01_solution() {
        let input = include_str!("../../inputs/day_12.txt").trim();

        assert_eq!(565, solve_1(input, true));
    }

    #[test]
    fn day_12_part_02_sample() {
        solve_2();
    }

    #[test]
    fn day_12_part_02_solution() {
        solve_2();
    }
}
