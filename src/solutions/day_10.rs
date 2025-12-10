use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::sync::LazyLock;
use rayon::prelude::*;
use z3::ast::Int;
use z3::Optimize;

pub fn solve_1(machines: &[&str]) -> u32 {
    machines
        .par_iter()
        .map(|machine| Machine::new(machine))
        .map(|machine| machine.min_presses_lights())
        .sum()
}

pub fn solve_2(machines: &[&str]) -> u64 {
    machines
        .par_iter()
        .map(|machine| Machine::new(machine))
        .map(|machine| machine.min_presses_joltages())
        .sum()
}

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

impl Machine {
    pub fn new(machine: &str) -> Self {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^\[(?<lights>[.#]+)] (?<buttons>(?:\((?:\d+,?)+\) ?)+) \{(?<joltages>(?:\d+,?)+)}$").unwrap()
        });

        let caps = RE.captures(machine).unwrap();
        let lights = caps["lights"]
            .chars()
            .map(|light| match light {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            })
            .collect_vec();
        let buttons = caps["buttons"]
            .split(" ")
            .map(|button| {
                button[1..button.len() - 1]
                    .split(',')
                    .map(|idx| idx.parse().unwrap())
                    .collect()
            })
            .collect();
        let joltages = caps["joltages"]
            .split(',')
            .map(|joltage| joltage.parse().unwrap())
            .collect();

        Self {
            lights,
            buttons,
            joltages,
        }
    }

    pub fn min_presses_lights(&self) -> u32 {
        let start = vec![false; self.lights.len()];
        let goal = self.lights.clone();

        let mut to_visit = VecDeque::from(vec![(start, 0)]);
        let mut visited = FxHashSet::default();

        while let Some((state, presses)) = to_visit.pop_front() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());

            if state == goal {
                return presses;
            }

            for button in &self.buttons {
                let mut next_state = state.clone();
                for idx in button {
                    next_state[*idx] = !next_state[*idx];
                }
                to_visit.push_back((next_state, presses + 1));
            }
        }

        unreachable!()
    }

    pub fn min_presses_joltages(&self) -> u64 {
        // Collect which button indexes influence which joltage index
        let buttons_per_joltage = {
            let grouped = self
                .buttons
                .iter()
                .enumerate()
                .flat_map(|(button_idx, button)| {
                    button.iter().map(move |&joltage_idx| (joltage_idx, button_idx))
                })
                .into_group_map();

            (0..self.joltages.len())
                .map(|joltage_idx| grouped.get(&joltage_idx).cloned().unwrap_or_default())
                .collect_vec()
        };

        // Setup variables
        let opt = Optimize::new();
        let total_presses = Int::fresh_const("total_presses");
        let all_presses = (0..self.buttons.len())
            .map(|idx| Int::fresh_const(&format!("button_{idx}")))
            .collect_vec();

        // Define constraints
        all_presses
            .iter()
            .for_each(|presses| opt.assert(&presses.ge(0)));
        (0..self.joltages.len())
            .for_each(|joltage_idx| {
                let joltage = self.joltages[joltage_idx];
                let buttons = buttons_per_joltage[joltage_idx]
                    .iter()
                    .map(|&button_idx| all_presses[button_idx].clone())
                    .collect_vec();
                opt.assert(&Int::add(&buttons).eq(Int::from_u64(joltage)));
            });
        opt.assert(&total_presses.eq(Int::add(&all_presses)));

        // Solve and return the result
        opt.minimize(&total_presses);
        opt.check(&[]);
        opt.get_model()
            .unwrap()
            .eval(&total_presses, true)
            .and_then(|total| total.as_u64())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_10_part_01_sample() {
        let sample = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        ];

        assert_eq!(7, solve_1(&sample));
    }

    #[test]
    fn day_10_part_01_solution() {
        let input = include_str!("../../inputs/day_10.txt")
            .lines()
            .collect_vec();

        assert_eq!(385, solve_1(&input));
    }

    #[test]
    fn day_10_part_02_sample() {
        let sample = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        ];

        assert_eq!(33, solve_2(&sample));
    }

    #[test]
    fn day_10_part_02_solution() {
        let input = include_str!("../../inputs/day_10.txt")
            .lines()
            .collect_vec();

        assert_eq!(16_757, solve_2(&input));
    }
}
