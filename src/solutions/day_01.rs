use itertools::Itertools;

pub fn solve_1(rotations: &[&str]) -> u32 {
    let rotations = rotations
        .iter()
        .map(|rotation| Rotation::new(rotation))
        .collect_vec();

    let mut position = 50;
    let mut zero_positions = 0;

    for rotation in &rotations {
        for _ in 0..rotation.distance {
            match rotation.direction {
                Direction::Left => {
                    if position == 0 {
                        position = 99;
                    } else {
                        position -= 1;
                    }
                }
                Direction::Right => {
                    if position == 99 {
                        position = 0;
                    } else {
                        position += 1;
                    }
                }
            }
        }

        if position == 0 {
            zero_positions += 1;
        }
    }

    zero_positions
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Rotation {
    direction: Direction,
    distance: u32,
}

impl Rotation {
    fn new(s: &str) -> Self {
        let direction = match &s[..=0] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };
        let distance = s[1..]
            .parse()
            .expect(&format!("Invalid rotation distance: {}", s));

        Self {
            direction,
            distance,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_01_part_01_sample() {
        let sample = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        assert_eq!(3, solve_1(&sample));
    }

    #[test]
    fn day_01_part_01_solution() {
        let input = include_str!("../../inputs/day_01.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_123, solve_1(&input));
    }
}
