use std::error::Error;
use std::io::prelude::*;

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn find_peaks(map: &Vec<Vec<u8>>, trailhead: (usize, usize)) -> usize {
    let height = map.len();
    let width = map[0].len();
    let valid = |c: (usize, usize)| c.0 < width && c.1 < height;

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut n_peaks = 0;
    queue.push_back(trailhead);
    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        for d in vec![(x + 1, x), (x - 1, y), (x, y - 1), (x, y + 1)] {
            if valid(d) && map[d.1][d.0] == map[y][x] + 1 {
                queue.push_back(d);
            }
        }
        if map[y][x] == 9 {
            n_peaks += 1;
        }
        visited.insert((x, y));
    }
    n_peaks
}

fn find_trails(map: &Vec<Vec<u8>>, (x, y): (usize, usize)) -> usize {
    if map[y][x] == 9 {
        1
    } else {
        let height = map.len();
        let width = map[0].len();
        let valid = |c: (usize, usize)| c.0 < width && c.1 < height;
        vec![(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)]
            .iter()
            .filter(|&&d| valid(d) && map[d.1][d.0] == map[y][x] + 1)
            .map(|d| find_trails(&map, *d))
            .sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut v = Vec::<Vec<u8>>::new();
    for line in stdin.lines() {
        let line = line?;
        v.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec(),
        );
    }
    let height = v.len();
    let width = v[0].len();

    let trailheads = (0..width)
        .cartesian_product(0..height)
        .filter(|(x, y)| v[*y][*x] == 0)
        .collect_vec();

    println!(
        "Total score of hiking map: {}",
        trailheads.iter().map(|h| find_peaks(&v, *h)).sum::<usize>()
    );

    println!(
        "Total rating of hiking map: {}",
        trailheads
            .iter()
            .map(|h| find_trails(&v, *h))
            .sum::<usize>()
    );

    return Ok(());
}
