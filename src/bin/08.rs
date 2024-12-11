use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::prelude::*;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use coordinates::two_dimensional::Vector2;
type Point = Vector2<isize>;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut antennae = HashMap::<char, HashSet<Point>>::new();
    let mut size = (0, 0);
    for (row, line) in stdin.lines().enumerate() {
        for (col, c) in line?.chars().enumerate() {
            size = (row + 1, col + 1);
            if !c.is_alphanumeric() {
                continue;
            }
            if let Some(r) = antennae.get_mut(&c) {
                r.insert((row as isize, col as isize).into());
            } else {
                antennae.insert(c, HashSet::from([(row as isize, col as isize).into()]));
            }
        }
    }
    let size = (size.0 as isize, size.1 as isize);

    let validate_antinode = |n: Point| {
        if 0 <= n.x && n.x < size.0 as isize && 0 <= n.y && n.y < size.1 {
            Some(n)
        } else {
            None
        }
    };

    let mut antinodes = HashSet::<Point>::new();
    let mut antinodes_harmonics = HashSet::<Point>::new();

    let mut add_antinode = |n| antinodes.insert(n);
    let mut add_resonant_antinodes = |start: Point, delta: Point| {
        std::iter::repeat(delta).fold_while(start, |n, d| {
            antinodes_harmonics.insert(n);
            validate_antinode(n + d).map(Continue).unwrap_or(Done(n))
        })
    };
    for (_, positions) in antennae.iter() {
        for pair in positions.iter().combinations(2) {
            let (a, b) = (*pair[0], *pair[1]);
            let d = a - b;

            validate_antinode(a + d).map(&mut add_antinode);
            validate_antinode(b - d).map(&mut add_antinode);

            let _ = &mut add_resonant_antinodes(a, d);
            let _ = &mut add_resonant_antinodes(b, -d);
        }
    }

    println!("Antinodes: {}", antinodes.len());
    println!("Antinodes (with harmonics): {}", antinodes_harmonics.len());

    return Ok(());
}
