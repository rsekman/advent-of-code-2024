use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

type DijkstraCost = usize;
#[derive(Eq, PartialEq, Debug)]
struct DijkstraNode<T: Hash + Eq + PartialEq + Debug> {
    dist: DijkstraCost,
    pos: T,
}

impl<T: Hash + Eq + PartialEq + Debug> Ord for DijkstraNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}
impl<T: Hash + Eq + PartialEq + Debug> PartialOrd for DijkstraNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}

// Dijkstra for shortest path between start and end
//#![feature(let_chains)]
pub fn dijkstra<T, NF>(start: &T, end: &T, mut neighbors: NF) -> Option<DijkstraCost>
where
    T: Copy + Hash + Eq + PartialEq + Debug,
    NF: FnMut(&T) -> Vec<(T, DijkstraCost)>,
{
    let mut queue = BinaryHeap::<DijkstraNode<T>>::new();
    let mut dists = HashMap::<T, usize>::new();

    queue.push(DijkstraNode {
        dist: 0,
        pos: *start,
    });

    while let Some(DijkstraNode { dist, pos }) = queue.pop() {
        if pos == *end {
            return Some(dist);
        }
        if dists.get(&pos).map(|d| dist > *d).unwrap_or(false) {
            continue;
        }
        for (n, d) in neighbors(&pos) {
            let m = DijkstraNode {
                dist: dist + d,
                pos: n,
            };
            //println!("{m:?}");
            if dists.get(&n).map(|d| m.dist < *d).unwrap_or(true) {
                dists.insert(m.pos, m.dist);
                queue.push(m);
            }
        }
    }

    // Unreachable target
    None
}
