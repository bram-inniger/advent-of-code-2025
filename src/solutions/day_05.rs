use itertools::Itertools;

pub fn solve_1(database: &str) -> usize {
    let database = Database::new(database);

    database
        .ingredients
        .iter()
        .filter(|ingredient| {
            database
                .fresh_ranges
                .iter()
                .any(|range| range.contains(ingredient))
        })
        .count()
}

pub fn solve_2(database: &str) -> u64 {
    Database::new(database)
        .fresh_ranges
        .iter()
        .map(|range| range.len())
        .sum()
}

type Ingredient = u64;

#[derive(Debug, Clone)]
struct Database {
    fresh_ranges: Vec<FreshRange>,
    ingredients: Vec<Ingredient>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct FreshRange {
    start: Ingredient,
    end: Ingredient,
}

impl Database {
    fn new(database: &str) -> Self {
        let (fresh_ranges, ingredients) = database.split_once("\n\n").unwrap();

        let fresh_ranges = fresh_ranges
            .lines()
            .map(|fresh_range| {
                fresh_range
                    .split_once("-")
                    .map(|(start, end)| FreshRange {
                        start: start.parse().unwrap(),
                        end: end.parse().unwrap(),
                    })
                    .unwrap()
            })
            .sorted_by_key(|range| range.start)
            .collect_vec();

        let mut merged_ranges = vec![];
        let mut current_range = fresh_ranges[0];

        for range in fresh_ranges.iter().skip(1) {
            let merged = current_range.sorted_merge(range);
            match merged {
                Some(merged) => current_range = merged,
                None => {
                    merged_ranges.push(current_range);
                    current_range = *range;
                }
            }
        }
        merged_ranges.push(current_range);

        let ingredients = ingredients
            .lines()
            .map(|ingredient| ingredient.parse().unwrap())
            .collect();

        Self {
            fresh_ranges: merged_ranges,
            ingredients,
        }
    }
}

impl FreshRange {
    pub fn sorted_merge(&self, other: &Self) -> Option<Self> {
        if other.start <= self.end {
            Some(Self {
                start: self.start,
                end: self.end.max(other.end),
            })
        } else {
            None
        }
    }

    pub fn contains(&self, ingredient: &Ingredient) -> bool {
        ingredient >= &self.start && ingredient <= &self.end
    }

    pub fn len(&self) -> u64 {
        self.end - self.start + 1
    }
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

    #[test]
    fn day_05_part_02_sample() {
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

        assert_eq!(14, solve_2(sample));
    }

    #[test]
    fn day_05_part_02_solution() {
        let input = include_str!("../../inputs/day_05.txt").trim();

        assert_eq!(336_173_027_056_994, solve_2(input));
    }
}
