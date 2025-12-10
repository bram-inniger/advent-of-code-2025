use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::sync::LazyLock;

pub fn solve_1(machines: &[&str]) -> u32 {
    machines
        .iter()
        .map(|machine| Machine::new(machine))
        .map(|machine| machine.min_presses())
        .sum()
}

#[derive(Debug, Clone)]
struct Machine {
    light_diagram: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    _joltages: Vec<u32>,
}

impl Machine {
    pub fn new(machine: &str) -> Self {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^\[(?<light_diagram>[.#]+)] (?<buttons>(?:\((?:\d+,?)+\) ?)+) \{(?<joltages>(?:\d+,?)+)}$").unwrap()
        });

        let caps = RE.captures(machine).unwrap();
        let light_diagram = caps["light_diagram"]
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
        let _joltages = caps["joltages"]
            .split(',')
            .map(|joltage| joltage.parse().unwrap())
            .collect();

        Self {
            light_diagram,
            buttons,
            _joltages,
        }
    }

    pub fn min_presses(&self) -> u32 {
        let start = vec![false; self.light_diagram.len()];
        let goal = self.light_diagram.clone();

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
}
