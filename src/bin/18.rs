use std::error::Error;
use std::io::prelude::*;

use std::collections::BTreeSet;

use nom::{
    character::complete::{char, newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use aoclib::dijkstra::dijkstra;
use coordinates::two_dimensional::Vector2;
use num::traits::{CheckedAdd, CheckedSub};

type Point = Vector2<usize>;

fn neighbors(p: &Point, (w, h): (usize, usize)) -> Vec<Point> {
    vec![
        p.checked_add(&(1, 0).into()),
        p.checked_sub(&(1, 0).into()),
        p.checked_add(&(0, 1).into()),
        p.checked_sub(&(0, 1).into()),
    ]
    .iter()
    .filter_map(|q| *q)
    .filter(|q| q.x <= w && q.y <= h)
    .collect()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(
        newline,
        map(separated_pair(u64, char(','), u64), |(x, y)| Point {
            x: x as usize,
            y: y as usize,
        }),
    )(input)
}

fn accessible(
    pos: &Point,
    bounds: (usize, usize),
    blocked: &BTreeSet<Point>,
) -> Vec<(Point, usize)> {
    neighbors(&pos, bounds)
        .iter()
        .filter(|n| !blocked.contains(&n))
        .map(|p| (*p, 1))
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, bytes) = parse_input(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let (w, h) = (70, 70);

    let n_bytes = 1024;
    let map = BTreeSet::from_iter(bytes.iter().take(n_bytes).cloned());

    let start: Point = (0, 0).into();
    let end: Point = (w, h).into();
    let dist =
        dijkstra(&start, &end, |p| accessible(p, (w, h), &map)).ok_or("No path to the exit!")?;

    println!("The shortest path is {dist:?} steps long.");

    let ns = Vec::from_iter(0..bytes.len());
    let cutoff = ns.as_slice().partition_point(|n| {
        dijkstra(&start, &end, |p| {
            accessible(
                p,
                (w, h),
                &BTreeSet::from_iter(bytes.iter().take(*n).cloned()),
            )
        })
        .is_some()
    }) - 1;
    println!(
        "The first byte that cuts off the path to the exit is #{cutoff} {}.",
        bytes[cutoff]
    );

    return Ok(());
}
