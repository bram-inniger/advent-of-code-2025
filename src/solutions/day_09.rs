use itertools::Itertools;

pub fn solve_1(red_tiles: &[&str]) -> u64 {
    red_tiles
        .iter()
        .map(|line| RedTile::new(line))
        .combinations(2)
        .map(|pair| pair[0].rectangle_to(&pair[1]))
        .max()
        .unwrap()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RedTile {
    x: u64,
    y: u64,
}

impl RedTile {
    pub fn new(line: &str) -> Self {
        let (x, y) = line
            .split_once(',')
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        Self { x, y }
    }

    pub fn rectangle_to(&self, other: &Self) -> u64 {
        let width = self.x.abs_diff(other.x) + 1;
        let height = self.y.abs_diff(other.y) + 1;
        
        width * height
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_09_part_01_sample() {
        let sample = vec!["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

        assert_eq!(50, solve_1(&sample));
    }

    #[test]
    fn day_09_part_01_solution() {
        let input = include_str!("../../inputs/day_09.txt")
            .lines()
            .collect_vec();

        assert_eq!(4_737_096_935, solve_1(&input));
    }
}
