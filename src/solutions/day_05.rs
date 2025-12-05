use itertools::Itertools;

pub fn solve_1(database: &str) -> usize {
    let (fresh_ranges, ingredients) = database.split_once("\n\n").unwrap();
    let fresh_ranges = fresh_ranges
        .lines()
        .map(|range| {
            let (start, end) = range
                .split_once("-")
                .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
                .unwrap();
            start..=end
        })
        .collect_vec();

    ingredients
        .lines()
        .map(|ingredient| ingredient.parse::<u64>().unwrap())
        .filter(|ingredient| fresh_ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_05_part_01_sample() {
        let sample = "\
            3-5\n\
            10-14\n\
            16-20\n\
            12-18\n\
            \n\
            1\n\
            5\n\
            8\n\
            11\n\
            17\n\
            32\
        ";

        assert_eq!(3, solve_1(sample));
    }

    #[test]
    fn day_05_part_01_solution() {
        let input = include_str!("../../inputs/day_05.txt").trim();

        assert_eq!(517, solve_1(input));
    }
}
