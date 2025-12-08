use crate::util::union_find::UnionFind;
use itertools::Itertools;
use num_traits::ToPrimitive;
use rustc_hash::FxHashMap;

pub fn solve_1(junctions: &[&str], nr_junctions: usize) -> usize {
    let junctions = junctions
        .iter()
        .map(|line| Position::new(line))
        .collect_vec();
    let shortest_pairs = junctions[..junctions.len() - 1]
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(idx, junction)| {
            junctions[idx + 1..]
                .iter()
                .map(move |&other| (junction, other, junction.distance_to(&other)))
        })
        .sorted_by_key(|(_, _, distance)| *distance)
        .take(nr_junctions)
        .collect_vec();
    let junctions_to_idx = junctions
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, junction)| (junction, idx))
        .collect::<FxHashMap<_, _>>();

    let mut uf = UnionFind::new(junctions.len());
    for (from, to, _) in shortest_pairs {
        uf.union(junctions_to_idx[&from], junctions_to_idx[&to]);
    }
    uf.sets()
        .iter()
        .map(|set| set.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: u32,
    y: u32,
    z: u32,
}

impl Position {
    pub fn new(position: &str) -> Self {
        position
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .map(|(x, y, z)| Position { x, y, z })
            .unwrap()
    }

    pub fn distance_to(&self, other: &Self) -> u32 {
        let dx = f64::from(self.x) - f64::from(other.x);
        let dy = f64::from(self.y) - f64::from(other.y);
        let dz = f64::from(self.z) - f64::from(other.z);

        (dx * dx + dy * dy + dz * dz)
            .sqrt()
            .round()
            .to_u32()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_08_part_01_sample() {
        let sample = vec![
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ];

        assert_eq!(40, solve_1(&sample, 10));
    }

    #[test]
    fn day_08_part_01_solution() {
        let input = include_str!("../../inputs/day_08.txt")
            .lines()
            .collect_vec();

        assert_eq!(83_520, solve_1(&input, 1_000));
    }
}
