use std::error::Error;
use std::io::prelude::*;

use rayon::prelude::*;

use aoclib::{
    dijkstra::{dijkstra, DijkstraPath},
    grid::{neighbors, IPoint},
};
use std::collections::{BTreeMap, BTreeSet};

fn racetrack_neighbors(p: &IPoint, accessible: &BTreeSet<IPoint>) -> Vec<(IPoint, usize)> {
    neighbors(p)
        .iter()
        .filter(|n| accessible.contains(n))
        .map(|n| (*n, 1))
        .collect::<Vec<_>>()
}

fn cheat(p: &IPoint, max_dist: usize) -> Vec<IPoint> {
    let max_dist = max_dist as isize;
    (-max_dist..=max_dist)
        .flat_map(|dx| {
            ((-max_dist + dx.abs())..=(max_dist - dx.abs())).map(move |dy| *p + (dx, dy).into())
        })
        .collect()
}

fn cheats(
    path: &DijkstraPath<IPoint>,
    dist_map: &BTreeMap<IPoint, usize>,
    cheat_dist: usize,
    cutoff: usize,
) -> usize {
    let cutoff = cutoff as isize;
    let v: Vec<_> = path.path.clone().into();
    v.par_iter()
        .map(|p| {
            cheat(&p.pos, cheat_dist)
                .par_iter()
                .map(|c| {
                    if let Some(d) = dist_map.get(&c) {
                        let IPoint { x: dx, y: dy } = p.pos - *c;
                        let cheat_time = dx.abs() + dy.abs();
                        (d.wrapping_sub(p.dist) as isize) - cheat_time
                    } else {
                        0
                    }
                })
                .filter(|t| *t >= cutoff)
                .count()
        })
        .sum::<usize>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut accessible = BTreeSet::<IPoint>::new();
    let mut start: Option<IPoint> = None;
    let mut end: Option<IPoint> = None;

    for (row, line) in stdin.lines().enumerate() {
        let line = line?;
        for (col, c) in line.chars().enumerate() {
            let col = col as isize;
            let row = row as isize;
            let mut mark_accessible = || accessible.insert((col, row).into());
            match c {
                '.' => {
                    mark_accessible();
                }
                'S' => {
                    mark_accessible();
                    start = Some((col, row).into())
                }
                'E' => {
                    mark_accessible();
                    end = Some((col, row).into())
                }
                _ => {}
            };
        }
    }

    let start = start.ok_or("Invalid input: no start position")?;
    let end = end.ok_or("Invalid input: no end position")?;

    let dists = dijkstra(&start, &end, |p| racetrack_neighbors(p, &accessible));
    let path = dists.get(&end).ok_or("Invalid input: no path to end.")?;
    println!("Best possible score: {:?}", path.dist());

    let dist_map: BTreeMap<IPoint, usize> = path.path.iter().map(|n| (n.pos, n.dist)).collect();

    let cheat_cutoff = 100;

    let cheat_dist_short = 2;
    let n_short_cheats = cheats(&path, &dist_map, cheat_dist_short, cheat_cutoff);
    println!("Number of length {cheat_dist_short} or less cheats that save {cheat_cutoff} ps or more: {n_short_cheats}.");

    let cheat_dist_long = 20;
    let n_long_cheats = cheats(&path, &dist_map, cheat_dist_long, cheat_cutoff);
    println!("Number of length {cheat_dist_long} or less cheats that save {cheat_cutoff} ps or more: {n_long_cheats}.");

    Ok(())
}
