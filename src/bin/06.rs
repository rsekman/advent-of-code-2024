use std::error::Error;
use std::io::prelude::*;

use itertools::Itertools;
use std::collections::BTreeSet;

type Point = (usize, usize);
type Map = Vec<Vec<char>>;

#[derive(Ord, Eq, PartialOrd, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn to_step(d: &Direction) -> (isize, isize) {
    match d {
        Direction::Up => (0, -1),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
    }
}

fn step((x, y): (usize, usize), d: &Direction) -> Option<Point> {
    let (dx, dy) = to_step(d);
    Option::zip(x.checked_add_signed(dx), y.checked_add_signed(dy))
}

fn rotate_clockwise(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_path(mut pos: Point, mut dir: Direction, map: &Map) -> (BTreeSet<(Point, Direction)>, bool) {
    let mut visited = BTreeSet::new();
    let is_loop = loop {
        if !visited.insert((pos, dir)) {
            break true;
        }
        if let Some((x, y)) = step(pos, &dir) {
            match map.get(y).and_then(|r| r.get(x)) {
                Some('#') => {
                    dir = rotate_clockwise(dir);
                    continue;
                }
                Some(_) => {}
                None => break false,
            }
            pos = (x, y)
        } else {
            break false;
        }
    };
    (visited, is_loop)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let map: Vec<Vec<char>> = stdin
        .lines()
        .map_ok(|l| l.chars().collect_vec())
        .try_collect()?;
    let pos: Point = match map
        .iter()
        .map(|r| r.iter().position(|&c| c == '^'))
        .find_position(Option::is_some)
    {
        Some((y, Some(x))) => Ok((x, y)),
        _ => Err("No starting position found"),
    }?;
    let dir = Direction::Up;

    let (original_path, orig_is_loop) = get_path(pos, dir, &map);
    let tiles_only = original_path
        .iter()
        .map(|(p, _)| *p)
        .collect::<BTreeSet<Point>>();
    println!(
        "Number of visited tiles: {} Loop? {}",
        tiles_only.len(),
        orig_is_loop
    );

    let mut loops = 0;
    let mut modified_map = map.clone();
    for &(x, y) in tiles_only.iter() {
        let prev = modified_map[y][x];
        modified_map[y][x] = '#';
        let (_, is_loop) = get_path(pos, dir, &modified_map);
        if is_loop {
            loops += 1;
        }
        modified_map[y][x] = prev;
    }
    println!("Number of loops that can be created: {}", loops);

    return Ok(());
}
