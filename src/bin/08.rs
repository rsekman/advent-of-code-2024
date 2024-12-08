use std::error::Error;
use std::io::prelude::*;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut antennae = BTreeMap::<char, BTreeSet<(isize, isize)>>::new();
    let mut size = (0, 0);
    for (row, line) in stdin.lines().enumerate() {
        for (col, c) in line?.chars().enumerate() {
            size = (row + 1, col + 1);
            if !c.is_alphanumeric() {
                continue;
            }
            if let Some(r) = antennae.get_mut(&c) {
                r.insert((row as isize, col as isize));
            } else {
                antennae.insert(c, BTreeSet::from([(row as isize, col as isize)]));
            }
        }
    }

    let validate_antinode = |r, c| {
        if 0 <= r && r < size.0 as isize && 0 <= c && c < size.1 as isize {
            Some((r, c))
        } else {
            None
        }
    };

    let mut antinodes = BTreeSet::<(isize, isize)>::new();
    let mut antinodes_harmonics = BTreeSet::<(isize, isize)>::new();

    let mut add_antinode = |n| antinodes.insert(n);
    let mut add_resonant_antinodes = |start, delta: (isize, isize)| {
        std::iter::repeat(delta).fold_while(start, |n, d| {
            antinodes_harmonics.insert(n);
            validate_antinode(n.0 + d.0, n.1 + d.1)
                .map(Continue)
                .unwrap_or(Done(n))
        })
    };
    for (_, positions) in antennae.iter() {
        for pair in positions.iter().combinations(2) {
            let (a, b) = (*pair[0], *pair[1]);
            let (dx, dy) = (a.0 - b.0, a.1 - b.1);

            validate_antinode(a.0 + dx, a.1 + dy).map(&mut add_antinode);
            validate_antinode(b.0 - dx, b.1 - dy).map(&mut add_antinode);

            let _ = &mut add_resonant_antinodes(a, (dx, dy));
            let _ = &mut add_resonant_antinodes(b, (-dx, -dy));
        }
    }

    println!("Antinodes: {}", antinodes.len());
    println!("Antinodes (with harmonics): {}", antinodes_harmonics.len());

    return Ok(());
}
