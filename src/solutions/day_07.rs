use rustc_hash::FxHashSet;

pub fn solve_1(manifold: &[&str]) -> usize {
    Manifold::new(manifold).beam_splits()
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
                to_visit.push(Position {
                    x: position.x + 1,
                    y: position.y,
                });
                to_visit.push(Position {
                    x: position.x - 1,
                    y: position.y,
                });
            } else {
                to_visit.push(Position {
                    x: position.x,
                    y: position.y + 1,
                });
            }
        }

        splitters.len()
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
}
