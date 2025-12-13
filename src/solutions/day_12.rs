use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::sync::LazyLock;

pub fn solve_1(summary: &str, heuristic: bool) -> usize {
    let parts = summary.split("\n\n").collect_vec();
    let shapes = parts[0..parts.len() - 1]
        .iter()
        .map(|shape| Shape::new(shape))
        .collect_vec();
    let regions = parts[parts.len() - 1]
        .split('\n')
        .map(|region| Region::new(region))
        .collect_vec();

    regions
        .par_iter()
        .filter(|region| {
            if heuristic {
                region.can_fit_heuristic()
            } else {
                region.can_fit(&shapes)
            }
        })
        .count()
}

pub fn solve_2() {
    // Decorate the North Pole
}

const DIM: usize = 3;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Shape {
    grid: [[bool; DIM]; DIM],
    coordinates: Vec<Coordinate>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Placement {
    Place,
    Remove,
}

impl Shape {
    pub fn new(shape: &str) -> Self {
        let grid = shape
            .lines()
            .skip(1)
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => unreachable!(),
                    })
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect_vec()
            .try_into()
            .unwrap();

        Self {
            grid,
            coordinates: Self::coordinates(&grid),
        }
    }

    pub fn transformations(&self) -> Vec<Self> {
        let mut transformations = vec![];
        let mut current = self.clone();

        for _ in 0..4 {
            transformations.push(current.clone());
            current = current.rotate_90();
        }

        current = self.flip_horizontal();

        for _ in 0..4 {
            transformations.push(current.clone());
            current = current.rotate_90();
        }

        transformations.sort();
        transformations.dedup();
        transformations
    }

    fn rotate_90(&self) -> Self {
        let mut new_grid = [[false; DIM]; DIM];
        for row in 0..DIM {
            for col in 0..DIM {
                new_grid[col][2 - row] = self.grid[row][col];
            }
        }
        Self {
            grid: new_grid,
            coordinates: Self::coordinates(&new_grid),
        }
    }

    fn flip_horizontal(&self) -> Self {
        let mut new_grid = [[false; DIM]; DIM];
        for row in 0..DIM {
            for col in 0..DIM {
                new_grid[row][2 - col] = self.grid[row][col];
            }
        }
        Self {
            grid: new_grid,
            coordinates: Self::coordinates(&new_grid),
        }
    }

    fn coordinates(grid: &[[bool; DIM]; DIM]) -> Vec<Coordinate> {
        (0..DIM)
            .flat_map(|row| (0..DIM).map(move |col| Coordinate { row, col }))
            .filter(|coord| grid[coord.row][coord.col])
            .collect()
    }
}

impl Region {
    pub fn new(region: &str) -> Self {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^(?<width>\d+)x(?<height>\d+): (?<shape_counts>.*)$").unwrap()
        });

        let caps = RE.captures(region).unwrap();

        let width = caps["width"].parse().unwrap();
        let height = caps["height"].parse().unwrap();
        let shape_counts = caps["shape_counts"]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            width,
            height,
            shape_counts,
        }
    }

    pub fn can_fit(&self, shapes: &[Shape]) -> bool {
        if self.can_fit_heuristic() {
            return true;
        }

        let mut grid = vec![vec![false; self.width]; self.height];
        let shapes_to_place = self
            .shape_counts
            .iter()
            .enumerate()
            .flat_map(|(shape_idx, &count)| {
                (0..count).map(move |_| shapes[shape_idx].transformations())
            })
            .collect_vec();

        self.backtrack(&mut grid, &shapes_to_place, 0)
    }

    pub fn can_fit_heuristic(&self) -> bool {
        let shapes_max = (self.width / DIM) * (self.height / DIM);
        let shapes_required = self.shape_counts.iter().sum::<usize>();

        shapes_max >= shapes_required
    }

    fn backtrack(
        &self,
        grid: &mut Vec<Vec<bool>>,
        shapes: &[Vec<Shape>],
        shape_idx: usize,
    ) -> bool {
        if shape_idx >= shapes.len() {
            return true;
        }

        for shape in &shapes[shape_idx] {
            for row_origin in 0..self.height {
                for col_origin in 0..self.width {
                    if self.can_place_shape(grid, &shape.coordinates, row_origin, col_origin) {
                        self.place_shape(
                            grid,
                            &shape.coordinates,
                            row_origin,
                            col_origin,
                            Placement::Place,
                        );

                        if self.backtrack(grid, shapes, shape_idx + 1) {
                            return true;
                        }

                        self.place_shape(
                            grid,
                            &shape.coordinates,
                            row_origin,
                            col_origin,
                            Placement::Remove,
                        );
                    }
                }
            }
        }

        false
    }

    fn can_place_shape(
        &self,
        grid: &[Vec<bool>],
        coords: &[Coordinate],
        row_origin: usize,
        col_origin: usize,
    ) -> bool {
        for Coordinate {
            row: row_delta,
            col: col_delta,
        } in coords
        {
            let row = row_origin + row_delta;
            let col = col_origin + col_delta;

            // Out of bounds
            if row >= self.height || col >= self.width {
                return false;
            }

            // Already occupied
            if grid[row][col] {
                return false;
            }
        }

        true
    }

    fn place_shape(
        &self,
        grid: &mut [Vec<bool>],
        coords: &[Coordinate],
        row_origin: usize,
        col_origin: usize,
        placement: Placement,
    ) {
        for Coordinate {
            row: row_delta,
            col: col_delta,
        } in coords
        {
            let row = row_origin + row_delta;
            let col = col_origin + col_delta;

            grid[row][col] = match placement {
                Placement::Place => true,
                Placement::Remove => false,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "Backtracking algorithm is very slow (+-45s on a MBP M1 Max)"]
    #[test]
    fn day_12_part_01_sample() {
        let sample = "\
            0:\n\
            ###\n\
            ##.\n\
            ##.\n\
            \n\
            1:\n\
            ###\n\
            ##.\n\
            .##\n\
            \n\
            2:\n\
            .##\n\
            ###\n\
            ##.\n\
            \n\
            3:\n\
            ##.\n\
            ###\n\
            ##.\n\
            \n\
            4:\n\
            ###\n\
            #..\n\
            ###\n\
            \n\
            5:\n\
            ###\n\
            .#.\n\
            ###\n\
            \n\
            4x4: 0 0 0 0 2 0\n\
            12x5: 1 0 1 0 2 2\n\
            12x5: 1 0 1 0 3 2\
        ";

        assert_eq!(2, solve_1(sample, false));
    }

    #[test]
    fn day_12_part_01_solution() {
        let input = include_str!("../../inputs/day_12.txt").trim();

        assert_eq!(565, solve_1(input, true));
    }

    #[test]
    fn day_12_part_02_sample() {
        solve_2();
    }

    #[test]
    fn day_12_part_02_solution() {
        solve_2();
    }
}
