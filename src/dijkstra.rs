extern crate alloc;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::vec::Vec;

use rayon::prelude::*;

use nonempty::{nonempty, NonEmpty};

type DijkstraCost = usize;
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct DijkstraNode<T: Hash + Eq + PartialEq + Copy + Clone> {
    pub pos: T,
    pub dist: usize,
}

impl<T: Hash + Eq + PartialEq + Copy + Clone> Ord for DijkstraNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}
impl<T: Hash + Eq + PartialEq + Copy + Clone> PartialOrd for DijkstraNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct DijkstraPath<T: Hash + Eq + PartialEq + Copy + Clone> {
    pub path: NonEmpty<DijkstraNode<T>>,
}

impl<T: Hash + Eq + PartialEq + Copy + Clone> DijkstraPath<T> {
    pub fn dist(&self) -> DijkstraCost {
        self.path.last().dist
    }
}

impl<T: Hash + Eq + PartialEq + Copy + Clone> Ord for DijkstraPath<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.last().cmp(&other.path.last())
    }
}

impl<T: Hash + Eq + PartialEq + Copy + Clone> PartialOrd for DijkstraPath<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.last().partial_cmp(&other.path.last())
    }
}

// Dijkstra for shortest path between start and end
pub fn dijkstra<T, NeighborFactory>(
    start: &T,
    end: &T,
    neighbors: NeighborFactory,
) -> HashMap<T, DijkstraPath<T>>
where
    T: Copy + Hash + Eq + PartialEq + Clone,
    NeighborFactory: Fn(&T) -> Vec<(T, DijkstraCost)>,
{
    dijkstra_by(start, |p| p == end, neighbors)
}

// Dijkstra for shortest path between start and first node to satisfy predicate end
pub fn dijkstra_by<T, EndPredicate, NeighborFactory>(
    start: &T,
    end: EndPredicate,
    neighbors: NeighborFactory,
) -> HashMap<T, DijkstraPath<T>>
where
    T: Copy + Hash + Eq + PartialEq + Clone,
    EndPredicate: Fn(&T) -> bool,
    NeighborFactory: Fn(&T) -> Vec<(T, DijkstraCost)>,
{
    let mut queue = BinaryHeap::<DijkstraNode<T>>::new();
    let mut dists = HashMap::<T, DijkstraPath<T>>::new();

    let s = DijkstraNode {
        pos: *start,
        dist: 0,
    };
    queue.push(s);
    dists.insert(*start, DijkstraPath { path: nonempty![s] });

    while let Some(p) = queue.pop() {
        let DijkstraNode { pos, dist } = p;
        if end(&pos) {
            break;
        }
        if dists.get(&pos).map(|d| d.dist() < dist).unwrap_or(false) {
            continue;
        }
        for (next, distance_to_next) in neighbors(&pos) {
            let next_node = DijkstraNode {
                pos: next,
                dist: dist + distance_to_next,
            };
            let cur_path = dists.get(&pos).unwrap();
            let path = dists.get(&next);
            if path.map(|pd| next_node.dist < pd.dist()).unwrap_or(true) {
                let mut new_path = cur_path.clone();
                new_path.path.push(next_node);
                queue.push(next_node);
                dists.insert(next_node.pos, new_path);
            }
        }
    }

    // Unreachable target
    dists
}

// Yen's algorithm for k shortest paths between start and end
pub fn yen<T, EndPredicate, NeighborFactory>(
    start: &T,
    end: &T,
    neighbors: NeighborFactory,
    max_k: Option<usize>,
    max_dist: Option<DijkstraCost>,
) -> Vec<DijkstraPath<T>>
where
    T: Copy + Hash + Eq + PartialEq + Clone + Sync + Send,
    NeighborFactory: Fn(&T) -> Vec<(T, DijkstraCost)> + Sync,
{
    let mut out: Vec<DijkstraPath<T>> = Vec::new();
    if let Some(path) = dijkstra(start, &end, &neighbors).get(&end) {
        out.push(path.clone());
    } else {
        return out;
    }
    let mut path_heap = BinaryHeap::<DijkstraPath<T>>::new();
    let mut paths = HashSet::<DijkstraPath<T>>::new();

    for j in 1.. {
        let last_path = &out[j - 1];
        let max_n = last_path.path.len() - 1;
        let new_paths: Vec<_> = (0..max_n)
            .into_par_iter()
            .filter_map(|n| {
                let root = last_path
                    .path
                    .iter()
                    .cloned()
                    .take(n + 1)
                    .collect::<Vec<_>>();
                let mut removed_edges = HashSet::<(T, T)>::new();
                for p in &out {
                    if *p.path.iter().take(n + 1).cloned().collect::<Vec<_>>() == root {
                        if let Some(q) = p.path.get(n)
                            && let Some(w) = p.path.get(n + 1)
                        {
                            removed_edges.insert((q.pos, w.pos));
                        }
                    }
                }
                let removed_nodes: HashSet<T> = root
                    .iter()
                    .map(|DijkstraNode { pos, dist: _ }| pos)
                    .take(n)
                    .cloned()
                    .collect();

                let subgraph = for<'a> |p: &'a T| -> Vec<(T, DijkstraCost)> {
                    neighbors(p)
                        .iter()
                        .filter(|(q, _)| {
                            !removed_nodes.contains(q)
                                && !removed_edges.contains(&(*p, *q))
                                && !removed_edges.contains(&(*q, *p))
                        })
                        .cloned()
                        .collect()
                };
                let spur_node = last_path.path[n].pos;

                if let Some(spur) = dijkstra(&spur_node, &end, subgraph).get(&end) {
                    let root_dist = root.last().map(|n| n.dist).unwrap_or(0);
                    let mut total_path = root.clone();
                    total_path.extend(spur.path.iter().skip(1).map(
                        |DijkstraNode { pos, dist }| DijkstraNode {
                            pos: *pos,
                            dist: dist + root_dist,
                        },
                    ));
                    let total_path = DijkstraPath {
                        path: NonEmpty::from_vec(total_path).unwrap(),
                    };
                    Some(total_path)
                } else {
                    None
                }
            })
            .collect();
        for p in new_paths {
            if !paths.contains(&p) {
                paths.insert(p.clone());
                path_heap.push(p);
            }
        }
        if let Some(path) = path_heap.pop() {
            if let Some(d) = max_dist
                && path.dist() > d
            {
                break;
            }
            out.push(path);
        } else {
            break;
        }
        if let Some(k) = max_k
            && k == j + 1
        {
            break;
        }
    }

    out
}
