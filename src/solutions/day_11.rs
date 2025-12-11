use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn solve_1(wiring: &[&str]) -> u64 {
    let wiring = wiring
        .iter()
        .map(|line| {
            let (from, to) = line.split_once(": ").unwrap();
            let to = to.split_whitespace().collect_vec();
            (from, to)
        })
        .collect::<FxHashMap<_, _>>();

    nr_paths("you", "out", &wiring, &mut FxHashMap::default())
}

fn nr_paths<'a>(
    from: &'a str,
    to: &str,
    wiring: &FxHashMap<&str, Vec<&'a str>>,
    memo: &mut FxHashMap<&'a str, u64>,
) -> u64 {
    if let Some(&count) = memo.get(from) {
        return count;
    }
    if from == to {
        return 1;
    }

    let nr_paths = wiring[from]
        .iter()
        .map(|&next| nr_paths(next, to, wiring, memo))
        .sum();
    memo.insert(from, nr_paths);
    nr_paths
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
}
