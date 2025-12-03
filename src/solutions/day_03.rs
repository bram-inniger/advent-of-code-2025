use crate::util::BASE_10;
use itertools::Itertools;

pub fn solve_1(banks: &[&str]) -> u32 {
    banks
        .iter()
        .map(|bank| Bank::new(bank))
        .map(|bank| bank.max_joltage())
        .sum()
}

#[derive(Debug, Clone)]
struct Bank {
    batteries: Vec<u32>,
}

impl Bank {
    pub fn new(batteries: &str) -> Self {
        let batteries = batteries
            .chars()
            .map(|battery| battery.to_digit(BASE_10).unwrap())
            .collect_vec();
        Self { batteries }
    }

    pub fn max_joltage(&self) -> u32 {
        fn largest_battery(batteries: &[u32]) -> (usize, u32) {
            batteries
                .iter()
                .enumerate()
                .max_by(|(idx_1, bat_1), (idx_2, bat_2)| bat_1.cmp(bat_2).then(idx_2.cmp(idx_1)))
                .map(|(idx, &bat)| (idx, bat))
                .unwrap()
        }

        let (first_idx, first_battery) =
            largest_battery(&self.batteries[..self.batteries.len() - 1]);
        let (_, second_battery) = largest_battery(&self.batteries[first_idx + 1..]);

        first_battery * 10 + second_battery
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
}
