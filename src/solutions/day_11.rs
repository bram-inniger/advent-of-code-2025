use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn solve_1(wiring: &[&str]) -> u64 {
    Wiring::new(wiring).nr_paths("you", "out")
}

pub fn solve_2(wiring: &[&str]) -> u64 {
    Wiring::new(wiring).nr_paths_specific("svr", "out")
}

#[derive(Debug, Clone)]
struct Wiring<'a> {
    wires: FxHashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Wiring<'a> {
    pub fn new(wiring: &[&'a str]) -> Self {
        let wires = wiring
            .iter()
            .map(|line| {
                let (from, to) = line.split_once(": ").unwrap();
                let to = to.split_whitespace().collect_vec();
                (from, to)
            })
            .collect::<FxHashMap<_, _>>();
        Self { wires }
    }

    pub fn nr_paths(&self, from: &'a str, to: &str) -> u64 {
        self.nr_paths_helper(from, to, &mut FxHashMap::default())
    }

    pub fn nr_paths_specific(&self, from: &'a str, to: &str) -> u64 {
        self.nr_paths_specific_helper(from, to, false, false, &mut FxHashMap::default())
    }

    fn nr_paths_helper(&self, from: &'a str, to: &str, memo: &mut FxHashMap<&'a str, u64>) -> u64 {
        if let Some(&count) = memo.get(from) {
            return count;
        }
        if from == to {
            return 1;
        }

        let nr_paths = self.wires[from]
            .iter()
            .map(|&next| self.nr_paths_helper(next, to, memo))
            .sum();
        memo.insert(from, nr_paths);
        nr_paths
    }

    fn nr_paths_specific_helper(
        &self,
        from: &'a str,
        to: &str,
        dac: bool,
        fft: bool,
        memo: &mut FxHashMap<(&'a str, bool, bool), u64>,
    ) -> u64 {
        if let Some(&count) = memo.get(&(from, dac, fft)) {
            return count;
        }
        if from == to {
            return if dac && fft { 1 } else { 0 };
        }

        let dac = dac || from == "dac";
        let fft = fft || from == "fft";
        let nr_paths = self.wires[from]
            .iter()
            .map(|&next| self.nr_paths_specific_helper(next, to, dac, fft, memo))
            .sum();
        memo.insert((from, dac, fft), nr_paths);
        nr_paths
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_11_part_01_sample() {
        let sample = vec![
            "aaa: you hhh",
            "you: bbb ccc",
            "bbb: ddd eee",
            "ccc: ddd eee fff",
            "ddd: ggg",
            "eee: out",
            "fff: out",
            "ggg: out",
            "hhh: ccc fff iii",
            "iii: out",
        ];

        assert_eq!(5, solve_1(&sample));
    }

    #[test]
    fn day_11_part_01_solution() {
        let input = include_str!("../../inputs/day_11.txt")
            .lines()
            .collect_vec();

        assert_eq!(782, solve_1(&input));
    }

    #[test]
    fn day_11_part_02_sample() {
        let sample = vec![
            "svr: aaa bbb",
            "aaa: fft",
            "fft: ccc",
            "bbb: tty",
            "tty: ccc",
            "ccc: ddd eee",
            "ddd: hub",
            "hub: fff",
            "eee: dac",
            "dac: fff",
            "fff: ggg hhh",
            "ggg: out",
            "hhh: out",
        ];

        assert_eq!(2, solve_2(&sample));
    }

    #[test]
    fn day_11_part_02_solution() {
        let input = include_str!("../../inputs/day_11.txt")
            .lines()
            .collect_vec();

        assert_eq!(401_398_751_986_160, solve_2(&input));
    }
}
