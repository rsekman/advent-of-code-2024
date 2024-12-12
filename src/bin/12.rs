use std::error::Error;
use std::io::prelude::*;

use itertools::{Itertools, MinMaxResult};
use num::traits::{CheckedAdd, CheckedSub};
use std::collections::{HashMap, HashSet};

use coordinates::two_dimensional::Vector2;
type Point = Vector2<isize>;

fn neighbors(p: Point) -> Vec<Point> {
    vec![
        p + (1isize, 0isize).into(),
        p + (-1isize, 0isize).into(),
        p + (0isize, 1isize).into(),
        p + (0isize, -1isize).into(),
    ]
}

fn connected_component(p: Point, garden: &HashMap<Point, char>) -> Option<HashSet<Point>> {
    let mut component = HashSet::new();
    let c = garden.get(&p);
    if !c.is_some() {
        return None;
    }
    let mut stack = Vec::new();
    stack.push(p);
    let c = c.unwrap();
    while let Some(q) = stack.pop() {
        component.insert(q);
        stack.extend(
            neighbors(q)
                .into_iter()
                .filter(|v| garden.get(&v) == Some(c) && !component.contains(&v)),
        );
    }
    Some(component)
}

fn perimiter(region: &HashSet<Point>) -> usize {
    let mut out = 0;
    for p in region.iter() {
        for q in neighbors(*p) {
            if !region.contains(&q) {
                out += 1
            }
        }
    }
    out
}

fn n_sides(region: &HashSet<Point>) -> usize {
    let (north, south) = match region.iter().minmax_by_key(|c| c.y) {
        MinMaxResult::OneElement(m) => (m.y, m.y),
        MinMaxResult::MinMax(n, s) => (n.y, s.y),
        MinMaxResult::NoElements => return 0,
    };
    let (west, east) = match region.iter().minmax_by_key(|c| c.x) {
        MinMaxResult::OneElement(m) => (m.x, m.x),
        MinMaxResult::MinMax(w, e) => (w.x, e.x),
        MinMaxResult::NoElements => return 0,
    };
    let hor_sides: usize = (north - 1..=south)
        .map(|y| {
            (west..=east)
                .chunk_by(|&x| {
                    match (
                        region.contains(&(x, y + 1).into()),
                        region.contains(&(x, y).into()),
                    ) {
                        (false, true) => -1isize,
                        (true, false) => 1,
                        _ => 0,
                    }
                })
                .into_iter()
                .map(|(s, _)| s.unsigned_abs())
                .sum::<usize>()
        })
        .sum();
    let ver_sides: usize = (west - 1..=east)
        .map(|x| {
            (north..=south)
                .chunk_by(|&y| {
                    match (
                        region.contains(&(x + 1, y).into()),
                        region.contains(&(x, y).into()),
                    ) {
                        (false, true) => -1isize,
                        (true, false) => 1,
                        _ => 0,
                    }
                })
                .into_iter()
                .map(|(s, _)| s.unsigned_abs())
                .sum::<usize>()
        })
        .sum();
    hor_sides + ver_sides
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut garden: HashMap<Point, char> = HashMap::new();
    for (row, line) in stdin.lines().enumerate() {
        let line = line?;
        garden.extend(line.chars().enumerate().map(|(col, c)| {
            (
                Point {
                    x: col as isize,
                    y: row as isize,
                },
                c,
            )
        }));
    }

    let mut visited: HashSet<Point> = HashSet::new();
    let mut components: Vec<HashSet<Point>> = Vec::new();
    for (k, _) in garden.iter() {
        if visited.contains(k) {
            continue;
        }
        if let Some(comp) = connected_component(*k, &garden) {
            for c in &comp {
                visited.insert(*c);
            }
            components.push(comp);
        }
    }

    let price: usize = components.iter().map(|c| c.len() * perimiter(&c)).sum();
    println!("Total price of fences: {price}");

    let price_sides = components
        .iter()
        .map(|c| c.len() * n_sides(&c))
        .collect_vec()
        .iter()
        .sum::<usize>();
    println!("Total price of fences (discounted): {price_sides}");

    return Ok(());
}
