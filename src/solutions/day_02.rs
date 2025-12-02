use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub fn solve_1(id_ranges: &str) -> u64 {
    let is_invalid = |id: &str| id.len() % 2 == 0 && id[..id.len() / 2] == id[id.len() / 2..];
    solve(id_ranges, is_invalid)
}

pub fn solve_2(id_ranges: &str) -> u64 {
    let is_invalid = |id: &str| {
        for rep_len in 1..=id.len() / 2 {
            if id.len() % rep_len != 0 {
                continue;
            }

            let rep_count = id.len() / rep_len;
            let repeated = id[0..rep_len].repeat(rep_count);

            if id == repeated {
                return true;
            }
        }

        false
    };
    solve(id_ranges, is_invalid)
}

fn solve(id_ranges: &str, is_invalid: impl Fn(&str) -> bool + Sync) -> u64 {
    id_ranges
        .split(",")
        .collect_vec()
        .par_iter()
        .map(|id_range| {
            let (start, end) = id_range
                .split_once("-")
                .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse().unwrap()))
                .unwrap();
            start..=end
        })
        .flat_map_iter(|range| {
            range.filter(|product_id| {
                let id = product_id.to_string();
                is_invalid(&id)
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_02_part_01_sample() {
        let sample = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
            824824821-824824827,2121212118-2121212124";

        assert_eq!(1_227_775_554, solve_1(sample));
    }

    #[test]
    fn day_02_part_01_solution() {
        let input = include_str!("../../inputs/day_02.txt").trim();

        assert_eq!(1_285_0231_731, solve_1(input));
    }

    #[test]
    fn day_02_part_02_sample() {
        let sample = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
            824824821-824824827,2121212118-2121212124";

        assert_eq!(4_174_379_265, solve_2(sample));
    }

    #[test]
    fn day_02_part_02_solution() {
        let input = include_str!("../../inputs/day_02.txt").trim();

        assert_eq!(24_774_350_322, solve_2(input));
    }
}
