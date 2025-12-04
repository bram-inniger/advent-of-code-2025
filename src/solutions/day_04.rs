use rustc_hash::FxHashSet;

pub fn solve_1(department: &[&str]) -> usize {
    let (_, rolls_removed) = Department::new(department).remove_paper_rolls();
    rolls_removed
}

pub fn solve_2(department: &[&str]) -> usize {
    let mut department = Department::new(department);
    let mut total_rolls_removed = 0;

    loop {
        let (new_department, rolls_removed) = department.remove_paper_rolls();

        if rolls_removed == 0 {
            return total_rolls_removed;
        }

        department = new_department;
        total_rolls_removed += rolls_removed;
    }
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

    pub fn remove_paper_rolls(&self) -> (Self, usize) {
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

        let paper_rolls = self
            .paper_rolls
            .iter()
            .copied()
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
                    >= 4
            })
            .collect::<FxHashSet<_>>();
        let old_rolls_count = self.paper_rolls.len();
        let new_rolls_count = paper_rolls.len();

        (Self { paper_rolls }, old_rolls_count - new_rolls_count)
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

    #[test]
    fn day_04_part_02_sample() {
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

        assert_eq!(43, solve_2(&sample));
    }

    #[test]
    fn day_04_part_02_solution() {
        let input = include_str!("../../inputs/day_04.txt")
            .lines()
            .collect_vec();

        assert_eq!(8_184, solve_2(&input));
    }
}
