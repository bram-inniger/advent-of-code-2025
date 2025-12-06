use itertools::Itertools;

pub fn solve_1(homework: &[&str]) -> u64 {
    let homework = homework
        .iter()
        .map(|line| line.split_whitespace().collect_vec())
        .collect_vec();
    let problems = (0..homework[0].len())
        .map(|idx| homework.iter().map(|line| line[idx]).collect_vec())
        .map(|problem| Problem::new(problem))
        .collect_vec();

    problems.iter().map(|problem| problem.solve()).sum()
}

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Operation {
    Addition,
    Multiplication,
}

impl Problem {
    pub fn new(problem: Vec<&str>) -> Self {
        let numbers = problem[..problem.len() - 1]
            .iter()
            .map(|number| number.parse().unwrap())
            .collect();
        let operation = match problem[problem.len() - 1] {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => unreachable!(),
        };
        Self { numbers, operation }
    }

    pub fn solve(&self) -> u64 {
        self.numbers.iter().fold(
            match self.operation {
                Operation::Addition => 0,
                Operation::Multiplication => 1,
            },
            |acc, number| match self.operation {
                Operation::Addition => acc + number,
                Operation::Multiplication => acc * number,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_06_part_01_sample() {
        let sample = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        assert_eq!(4_277_556, solve_1(&sample));
    }

    #[test]
    fn day_06_part_01_solution() {
        let input = include_str!("../../inputs/day_06.txt")
            .lines()
            .collect_vec();

        assert_eq!(5_552_221_122_013, solve_1(&input));
    }
}
