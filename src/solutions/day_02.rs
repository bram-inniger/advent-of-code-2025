use fancy_regex::Regex;
use itertools::Itertools;
use rayon::prelude::*;

pub fn solve_1(id_ranges: &str) -> u64 {
    solve(id_ranges, &Regex::new(r"^(.+)\1{1}$").unwrap())
}

pub fn solve_2(id_ranges: &str) -> u64 {
    solve(id_ranges, &Regex::new(r"^(.+)\1+$").unwrap())
}

fn solve(id_ranges: &str, re: &Regex) -> u64 {
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
            range.filter(|product_id| re.is_match(&product_id.to_string()).unwrap())
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
