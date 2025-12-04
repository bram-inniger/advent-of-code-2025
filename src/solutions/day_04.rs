use rustc_hash::FxHashSet;

pub fn solve_1(department: &[&str]) -> usize {
    Department::new(department).accessible_paper_rolls()
}

#[derive(Debug, Clone)]
struct Department {
    paper_rolls: FxHashSet<Position>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Department {
    pub fn new(grid: &[&str]) -> Self {
        let paper_rolls = grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (x as i32, y as i32, c))
            })
            .filter(|&(_, _, c)| c == '@')
            .map(|(x, y, _)| Position { x, y })
            .collect();
        Self { paper_rolls }
    }

    pub fn accessible_paper_rolls(&self) -> usize {
        let neighbour_deltas = vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        self.paper_rolls
            .iter()
            .filter(|Position { x, y }| {
                neighbour_deltas
                    .iter()
                    .filter(|(dx, dy)| {
                        let neighbour = Position {
                            x: x + dx,
                            y: y + dy,
                        };
                        self.paper_rolls.contains(&neighbour)
                    })
                    .count()
                    < 4
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_04_part_01_sample() {
        let sample = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];

        assert_eq!(13, solve_1(&sample));
    }

    #[test]
    fn day_04_part_01_solution() {
        let input = include_str!("../../inputs/day_04.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_363, solve_1(&input));
    }
}
