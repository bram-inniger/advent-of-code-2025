use itertools::Itertools;
use std::ops::Range;

pub fn solve_1(homework: &[&str]) -> u64 {
    solve(homework, Problem::classic_math)
}

pub fn solve_2(homework: &[&str]) -> u64 {
    solve(homework, Problem::cephalopod_math)
}

fn solve(homework: &[&str], math: impl Fn(&[&[char]], Operation) -> Problem) -> u64 {
    let homework = homework
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let operators = homework.last().unwrap();
    let operators = (0..operators.len())
        .filter(|&idx| operators[idx] != ' ')
        .map(|idx| {
            (
                idx,
                match operators[idx] {
                    '+' => Operation::Addition,
                    '*' => Operation::Multiplication,
                    _ => unreachable!(),
                },
            )
        })
        .collect_vec();

    (0..operators.len())
        .map(|idx| {
            let (start, operation) = operators[idx];
            let end = if idx < operators.len() - 1 {
                operators[idx + 1].0 - 1
            } else {
                homework[0].len()
            };

            (start..end, operation)
        })
        .map(|(Range { start, end }, operation)| {
            let numbers = &homework[..homework.len() - 1]
                .iter()
                .map(|line| &line[start..end])
                .collect_vec();
            math(numbers, operation)
        })
        .map(|problem| problem.solve())
        .sum()
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
    pub fn classic_math(numbers: &[&[char]], operation: Operation) -> Self {
        let numbers = numbers
            .iter()
            .map(|number| number.iter().collect::<String>().trim().parse().unwrap())
            .collect();
        Self { numbers, operation }
    }

    pub fn cephalopod_math(numbers: &[&[char]], operation: Operation) -> Self {
        let numbers = (0..numbers[0].len())
            .map(|idx| {
                numbers
                    .iter()
                    .map(|line| line[idx])
                    .collect::<String>()
                    .trim()
                    .parse()
                    .unwrap()
            })
            .collect();
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

    #[test]
    fn day_06_part_02_sample() {
        let sample = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        assert_eq!(3_263_827, solve_2(&sample));
    }

    #[test]
    fn day_06_part_02_solution() {
        let input = include_str!("../../inputs/day_06.txt")
            .lines()
            .collect_vec();

        assert_eq!(11_371_597_126_232, solve_2(&input));
    }
}
