pub fn solve_1(_: &[&str]) -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_01_part_01_sample() {
        let sample = vec![
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ];

        assert_eq!(42, solve_1(&sample));
    }

    #[test]
    fn day_01_part_01_solution() {
        let input = include_str!("../../inputs/day_01.txt")
            .lines()
            .collect_vec();

        assert_eq!(0, solve_1(&input));
    }
}
