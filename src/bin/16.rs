#![feature(let_chains)]
use std::error::Error;
use std::fmt;
use std::io::prelude::*;

use aoclib::{
    dijkstra::{dijkstra, dijkstra_by},
    grid::{clockwise, counterclockwise, step, CardinalDirection, UPoint},
};
use std::collections::BTreeSet;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Reindeer {
    pos: UPoint,
    orientation: CardinalDirection,
}

impl std::fmt::Debug for Reindeer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.pos.x,
            self.pos.y,
            match self.orientation {
                CardinalDirection::North => '^',
                CardinalDirection::East => '>',
                CardinalDirection::South => 'v',
                CardinalDirection::West => '<',
            }
        )
    }
}

fn neighbors(r: &Reindeer, accessible: &BTreeSet<UPoint>) -> Vec<(Reindeer, usize)> {
    let mut out = vec![
        (
            Reindeer {
                pos: r.pos,
                orientation: clockwise(r.orientation),
            },
            1000,
        ),
        (
            Reindeer {
                pos: r.pos,
                orientation: counterclockwise(r.orientation),
            },
            1000,
        ),
    ];
    if let Some(pos) = step(r.pos, r.orientation)
        && accessible.contains(&pos)
    {
        out.push((
            Reindeer {
                pos,
                orientation: r.orientation,
            },
            1,
        ))
    }
    out
}

fn neighbors_rev(r: &Reindeer, accessible: &BTreeSet<UPoint>) -> Vec<(Reindeer, usize)> {
    let mut out = vec![
        (
            Reindeer {
                pos: r.pos,
                orientation: clockwise(r.orientation),
            },
            1000,
        ),
        (
            Reindeer {
                pos: r.pos,
                orientation: counterclockwise(r.orientation),
            },
            1000,
        ),
    ];
    if let Some(pos) = step(r.pos, -r.orientation)
        && accessible.contains(&pos)
    {
        out.push((
            Reindeer {
                pos,
                orientation: r.orientation,
            },
            1,
        ))
    }
    out
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut accessible = BTreeSet::<UPoint>::new();
    let mut start: Option<UPoint> = None;
    let mut end: Option<UPoint> = None;

    for (row, line) in stdin.lines().enumerate() {
        let line = line?;
        for (col, c) in line.chars().enumerate() {
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

    let start = Reindeer {
        pos: start,
        orientation: CardinalDirection::East,
    };
    let end_pred = |r: &Reindeer| r.pos == end;
    let dirs = vec![
        CardinalDirection::North,
        CardinalDirection::East,
        CardinalDirection::South,
        CardinalDirection::West,
    ];

    let dists = dijkstra_by(&start, &end_pred, |r| neighbors(r, &accessible));
    let shortest = dirs
        .iter()
        .filter_map(|&orientation| {
            dists.get(&Reindeer {
                pos: end,
                orientation,
            })
        })
        .min_by_key(|p| p.dist())
        .ok_or("Invalid input: no path to finish.")?
        .dist();
    println!("Best possible score: {}", shortest);

    let mut on_shortest_path = BTreeSet::new();
    for d in dirs {
        let end_d = Reindeer {
            pos: end,
            orientation: d,
        };
        let dists = dijkstra(&start, &end_d, |r| neighbors(r, &accessible));
        let dist_to_end = dists.get(&end_d).unwrap().dist();

        let mut stack: Vec<(Reindeer, usize)> = Vec::new();
        if dist_to_end == shortest {
            on_shortest_path.insert(end_d.pos);
            stack.push((end_d, dist_to_end));
        }
        while let Some((node, dist)) = stack.pop() {
            for (n, d) in neighbors_rev(&node, &accessible) {
                if let Some(path) = dists.get(&n)
                    && path.dist() + d == dist
                {
                    stack.push((n, path.dist()));
                    on_shortest_path.insert(n.pos);
                }
            }
        }
    }
    println!(
        "Number of tiles on any shortest path: {}",
        on_shortest_path.len()
    );

    return Ok(());
}
