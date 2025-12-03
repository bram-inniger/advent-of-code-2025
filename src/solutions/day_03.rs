use crate::util::BASE_10;
use itertools::Itertools;

pub fn solve_1(banks: &[&str]) -> u64 {
    solve(banks, 2)
}

pub fn solve_2(banks: &[&str]) -> u64 {
    solve(banks, 12)
}

fn solve(banks: &[&str], batteries_count: usize) -> u64 {
    banks
        .iter()
        .map(|bank| Bank::new(bank))
        .map(|bank| bank.max_joltage(batteries_count))
        .sum()
}

#[derive(Debug, Clone)]
struct Bank {
    batteries: Vec<u64>,
}

impl Bank {
    pub fn new(batteries: &str) -> Self {
        let batteries = batteries
            .chars()
            .map(|battery| battery.to_digit(BASE_10).unwrap().into())
            .collect_vec();
        Self { batteries }
    }

    pub fn max_joltage(&self, batteries_count: usize) -> u64 {
        let mut joltage = 0;
        let mut start_idx = 0;
        let mut end_idx = self.batteries.len() - batteries_count + 1;

        for _ in 0..batteries_count {
            let (idx, battery) = self.batteries[start_idx..end_idx]
                .iter()
                .enumerate()
                .max_by(|(idx_1, bat_1), (idx_2, bat_2)| bat_1.cmp(bat_2).then(idx_2.cmp(idx_1)))
                .map(|(idx, &bat)| (idx, bat))
                .unwrap();

            joltage = joltage * 10 + battery;
            start_idx += idx + 1;
            end_idx += 1;
        }

        joltage
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_03_part_01_sample() {
        let sample = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(357, solve_1(&sample));
    }

    #[test]
    fn day_03_part_01_solution() {
        let input = include_str!("../../inputs/day_03.txt")
            .lines()
            .collect_vec();

        assert_eq!(16_973, solve_1(&input));
    }

    #[test]
    fn day_03_part_02_sample() {
        let sample = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(3_121_910_778_619, solve_2(&sample));
    }

    #[test]
    fn day_03_part_02_solution() {
        let input = include_str!("../../inputs/day_03.txt")
            .lines()
            .collect_vec();

        assert_eq!(168_027_167_146_027, solve_2(&input));
    }
}
