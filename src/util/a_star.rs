use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;

pub trait Graphable: Clone + Eq + Hash {
    type Context;

    fn neighbors(&self, context: &Self::Context) -> Vec<(Self, u32)>;

    fn heuristic(&self, target: &Self, context: &Self::Context) -> u32;

    fn is_target(&self, target: &Self) -> bool {
        self == target
    }
}

pub fn a_star<T: Graphable>(start: T, target: T, context: &T::Context) -> Option<(Vec<T>, u32)> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: FxHashMap<T, T> = FxHashMap::default();
    let mut g_scores: FxHashMap<T, u32> = FxHashMap::default();
    let mut visited: FxHashSet<T> = FxHashSet::default();

    let h_score = start.heuristic(&target, context);
    let g_score = 0;
    let f_score = g_score + h_score;

    g_scores.insert(start.clone(), g_score);
    open_set.push(Node {
        state: start.clone(),
        g_score,
        f_score,
    });

    while let Some(Node {
        state: current,
        g_score,
        ..
    }) = open_set.pop()
    {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());

        if current.is_target(&target) {
            return Some((reconstruct_path(came_from, current), g_score));
        }

        for (neighbor, cost) in current.neighbors(context) {
            if visited.contains(&neighbor) {
                continue;
            }

            let potentially_better_g = g_score + cost;

            if let Some(&existing_g) = g_scores.get(&neighbor) {
                if potentially_better_g >= existing_g {
                    continue;
                }
            }

            let h_score = neighbor.heuristic(&target, context);
            let g_score = potentially_better_g;
            let f_score = g_score + h_score;

            came_from.insert(neighbor.clone(), current.clone());
            g_scores.insert(neighbor.clone(), g_score);
            open_set.push(Node {
                state: neighbor,
                g_score,
                f_score,
            });
        }
    }

    None
}

#[derive(Clone)]
struct Node<T> {
    state: T,
    g_score: u32,
    f_score: u32,
}

impl<T> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering to make the `BinaryHeap` a min-heap
        other.f_score.cmp(&self.f_score)
    }
}

impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl<T> Eq for Node<T> {}

fn reconstruct_path<T: Graphable>(came_from: FxHashMap<T, T>, mut current: T) -> Vec<T> {
    let mut path = vec![current.clone()];

    while let Some(prev) = came_from.get(&current) {
        current = prev.clone();
        path.push(current.clone());
    }

    path.reverse();
    path
}
