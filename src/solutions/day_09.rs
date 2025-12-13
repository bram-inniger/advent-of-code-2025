use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::ops::Not;

pub fn solve_1(red_tiles: &[&str]) -> u64 {
    red_tiles
        .iter()
        .map(|line| RedTile::new(line))
        .combinations(2)
        .map(|rectangle| Rectangle::new(rectangle[0], rectangle[1]))
        .map(|rectangle| rectangle.area())
        .max()
        .unwrap()
}

pub fn solve_2(red_tiles: &[&str]) -> u64 {
    let red_tiles = red_tiles
        .iter()
        .map(|line| RedTile::new(line))
        .collect_vec();
    let edges = red_tiles
        .iter()
        .circular_tuple_windows::<(_, _)>()
        .map(|(from, to)| {
            if from.x == to.x {
                Edge::Vertical {
                    x: from.x,
                    y_1: from.y.min(to.y),
                    y_2: from.y.max(to.y),
                }
            } else if from.y == to.y {
                Edge::Horizontal {
                    x_1: from.x.min(to.x),
                    x_2: from.x.max(to.x),
                    y: from.y,
                }
            } else {
                unreachable!()
            }
        })
        .collect_vec();
    let rectangles = red_tiles
        .iter()
        .copied()
        .combinations(2)
        .map(|rectangle| Rectangle::new(rectangle[0], rectangle[1]))
        .collect_vec();
    let red_tiles = red_tiles.into_iter().collect();

    let mut max_area = 0;
    for rectangle in &rectangles {
        if rectangle.area() <= max_area {
            continue;
        }
        if rectangle.is_within(&edges, &red_tiles) {
            max_area = rectangle.area();
        }
    }

    max_area
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RedTile {
    x: u64,
    y: u64,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Edge {
    Horizontal { x_1: u64, x_2: u64, y: u64 },
    Vertical { x: u64, y_1: u64, y_2: u64 },
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Rectangle {
    top_left: RedTile,
    bottom_right: RedTile,
}

impl RedTile {
    pub fn new(line: &str) -> Self {
        let (x, y) = line
            .split_once(',')
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        Self { x, y }
    }

    pub fn is_within(&self, edges: &[Edge], red_tiles: &FxHashSet<RedTile>) -> bool {
        let on_red_tile = || red_tiles.contains(self);
        let on_edge = || {
            edges.iter().any(|edge| match edge {
                Edge::Horizontal { x_1, x_2, y } => {
                    &self.y == y && &self.x >= x_1 && &self.x <= x_2
                }
                Edge::Vertical { x, y_1, y_2 } => &self.x == x && &self.y >= y_1 && &self.y <= y_2,
            })
        };
        let crossed_edges = || {
            edges
                .iter()
                .filter(|edge| match edge {
                    Edge::Horizontal { .. } => false,
                    Edge::Vertical { x, y_1, y_2 } => {
                        x > &self.x && y_1 <= &self.y && &self.y < y_2
                    }
                })
                .count()
        };

        on_red_tile() || on_edge() || crossed_edges() % 2 == 1
    }
}

impl Edge {
    pub fn intersects(&self, edges: &[Edge]) -> bool {
        edges.iter().any(|other| match other {
            Edge::Horizontal { x_1, x_2, y } => match self {
                Edge::Horizontal { .. } => false,
                Edge::Vertical { x, y_1, y_2 } => x > x_1 && x < x_2 && y > y_1 && y < y_2,
            },
            Edge::Vertical { x, y_1, y_2 } => match self {
                Edge::Horizontal { x_1, x_2, y } => x > x_1 && x < x_2 && y > y_1 && y < y_2,
                Edge::Vertical { .. } => false,
            },
        })
    }
}

impl Rectangle {
    pub fn new(tile_a: RedTile, tile_b: RedTile) -> Self {
        let top_left = RedTile {
            x: tile_a.x.min(tile_b.x),
            y: tile_a.y.min(tile_b.y),
        };
        let bottom_right = RedTile {
            x: tile_a.x.max(tile_b.x),
            y: tile_a.y.max(tile_b.y),
        };
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn area(&self) -> u64 {
        let width = self.bottom_right.x - self.top_left.x + 1;
        let height = self.bottom_right.y - self.top_left.y + 1;

        width * height
    }

    pub fn is_within(&self, edges: &[Edge], red_tiles: &FxHashSet<RedTile>) -> bool {
        let corners_are_within = [
            RedTile {
                x: self.top_left.x,
                y: self.top_left.y,
            },
            RedTile {
                x: self.top_left.x,
                y: self.bottom_right.y,
            },
            RedTile {
                x: self.bottom_right.x,
                y: self.top_left.y,
            },
            RedTile {
                x: self.bottom_right.x,
                y: self.bottom_right.y,
            },
        ]
        .iter()
        .all(|corner| corner.is_within(edges, red_tiles));
        let intersect_edges = [
            Edge::Horizontal {
                x_1: self.top_left.x,
                x_2: self.bottom_right.x,
                y: self.top_left.y,
            },
            Edge::Vertical {
                x: self.top_left.x,
                y_1: self.top_left.y,
                y_2: self.bottom_right.y,
            },
            Edge::Horizontal {
                x_1: self.top_left.x,
                x_2: self.bottom_right.x,
                y: self.bottom_right.y,
            },
            Edge::Vertical {
                x: self.bottom_right.x,
                y_1: self.top_left.y,
                y_2: self.bottom_right.y,
            },
        ]
        .iter()
        .any(|edge| edge.intersects(edges));

        corners_are_within && intersect_edges.not()
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

    #[test]
    fn day_09_part_02_sample() {
        let sample = vec!["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

        assert_eq!(24, solve_2(&sample));
    }

    #[test]
    fn day_09_part_02_solution() {
        let input = include_str!("../../inputs/day_09.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_644_094_530, solve_2(&input));
    }
}
