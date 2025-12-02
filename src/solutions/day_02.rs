pub fn solve_1(id_ranges: &str) -> u64 {
    id_ranges
        .split(",")
        .map(|id_range| {
            let (start, end) = id_range
                .split_once("-")
                .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse().unwrap()))
                .unwrap();
            start..=end
        })
        .flat_map(|range| {
            range.filter(|product_id| {
                let id = product_id.to_string();
                id.len() % 2 == 0 && id[..id.len() / 2] == id[id.len() / 2..]
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
}
