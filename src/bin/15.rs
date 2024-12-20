use std::error::Error;
use std::io::prelude::*;

use coordinates::two_dimensional::Vector2;
use itertools::Itertools;

type Point = Vector2<isize>;

fn attempt_move(warehouse: &mut Vec<Vec<char>>, pos: Point, dir: Point) -> Point {
    let to = pos + dir;
    let mut p = to;
    while warehouse[p.y as usize][p.x as usize] == 'O' {
        p = p + dir;
    }
    if warehouse[p.y as usize][p.x as usize] == '#' {
        return pos;
    } else {
        warehouse[p.y as usize][p.x as usize] = 'O';
        warehouse[to.y as usize][to.x as usize] = '.';
        return to;
    }
}

fn attempt_move_wide(warehouse: &mut Vec<Vec<char>>, pos: Point, dir: Point) -> Point {
    let to = pos + dir;
    let mut p = to;
    while warehouse[p.y as usize][p.x as usize] == '['
        || warehouse[p.y as usize][p.x as usize] == ']'
    {
        p = p + dir;
    }
    if warehouse[p.y as usize][p.x as usize] == '#' {
        return pos;
    } else {
        warehouse[p.y as usize][p.x as usize] = 'O';
        warehouse[to.y as usize][to.x as usize] = '.';
        return to;
    }
}

fn gps(warehouse: &Vec<Vec<char>>) -> usize {
    warehouse
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, c)| if *c == 'O' { 100 * y + x } else { 0 })
        })
        .flatten()
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    let mut warehouse: Vec<Vec<char>> = Vec::new();
    let mut pos = Point { x: 0, y: 0 };
    for (r, line) in lock.lines().enumerate() {
        let line = line?;
        if line.len() == 0 {
            break;
        }
        if let Some(c) = line.chars().position(|x| x == '@') {
            pos = Point {
                x: c as isize,
                y: r as isize,
            };
        }
        warehouse.push(line.chars().collect());
    }
    warehouse[pos.y as usize][pos.x as usize] = '.';

    let mut instructions = Vec::<char>::new();
    let lock = stdin.lock();
    for line in lock.lines() {
        let line = line?;
        instructions.extend(line.chars());
    }

    for i in instructions.iter() {
        let dir = match i {
            '>' => Point { x: 1, y: 0 },
            '<' => Point { x: -1, y: 0 },
            '^' => Point { x: 0, y: -1 },
            'v' => Point { x: 0, y: 1 },
            _ => panic!("Invalid input"),
        };
        pos = attempt_move(&mut warehouse, pos, dir);
        warehouse[pos.y as usize][pos.x as usize] = '@';
        /*
        println!(
            "Move: {i}\n{}\n",
            warehouse.iter().map(|r| r.iter().join("")).join("\n")
        );
        warehouse[pos.y as usize][pos.x as usize] = '.';
        */
    }

    println!("GPS score: {}", gps(&warehouse));

    return Ok(());
}
