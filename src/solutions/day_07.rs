use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(manifold: &[&str]) -> usize {
    Manifold::new(manifold).beam_splits()
}

pub fn solve_2(manifold: &[&str]) -> u64 {
    Manifold::new(manifold).timeline_splits()
}

#[derive(Debug, Clone)]
struct Manifold {
    entry: Position,
    splitters: FxHashSet<Position>,
    depth: usize,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Manifold {
    pub fn new(manifold: &[&str]) -> Self {
        let entry = Position {
            x: manifold[0].find('S').unwrap(),
            y: 0,
        };
        let splitters = manifold
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '^' {
                        Some(Position { x, y })
                    } else {
                        None
                    }
                })
            })
            .collect();
        let depth = manifold.len();

        Self {
            entry,
            splitters,
            depth,
        }
    }

    pub fn beam_splits(&self) -> usize {
        let mut splitters = FxHashSet::default();
        let mut to_visit = vec![self.entry];
        let mut visited = FxHashSet::default();

        while let Some(position) = to_visit.pop() {
            if visited.contains(&position) || position.y >= self.depth {
                continue;
            }
            visited.insert(position);

            if self.splitters.contains(&position) {
                splitters.insert(position);
                to_visit.push(Position::of(position.x + 1, position.y));
                to_visit.push(Position::of(position.x - 1, position.y));
            } else {
                to_visit.push(Position::of(position.x, position.y + 1));
            }
        }

        splitters.len()
    }

    pub fn timeline_splits(&self) -> u64 {
        self.timeline_splits_helper(&self.entry, &mut FxHashMap::default())
    }

    fn timeline_splits_helper(
        &self,
        position: &Position,
        timelines_at: &mut FxHashMap<Position, u64>,
    ) -> u64 {
        if position.y == self.depth {
            return 1;
        }
        if timelines_at.contains_key(position) {
            return timelines_at[position];
        }

        let timelines = (if self.splitters.contains(position) {
            vec![
                Position::of(position.x - 1, position.y),
                Position::of(position.x + 1, position.y),
            ]
        } else {
            vec![Position::of(position.x, position.y + 1)]
        })
        .iter()
        .map(|p| self.timeline_splits_helper(p, timelines_at))
        .sum();

        timelines_at.insert(*position, timelines);
        timelines
    }
}

impl Position {
    fn of(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_07_part_01_sample() {
        let sample = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        assert_eq!(21, solve_1(&sample));
    }

    #[test]
    fn day_07_part_01_solution() {
        let input = include_str!("../../inputs/day_07.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_651, solve_1(&input));
    }

    #[test]
    fn day_07_part_02_sample() {
        let sample = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        assert_eq!(40, solve_2(&sample));
    }

    #[test]
    fn day_07_part_02_solution() {
        let input = include_str!("../../inputs/day_07.txt")
            .lines()
            .collect_vec();

        assert_eq!(108_924_003_331_749, solve_2(&input));
    }
}
