use std::error::Error;
use std::io::prelude::*;

use ::std::cmp::Ordering::*;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i32, newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::collections::{BTreeMap, BTreeSet};

use coordinates::two_dimensional::Vector2;
type Coordinates = Vector2<i32>;

#[derive(Debug)]
struct Robot {
    p: Coordinates,
    v: Coordinates,
}

fn coords(input: &str) -> IResult<&str, Coordinates> {
    map(separated_pair(i32, char(','), i32), |(x, y)| Coordinates {
        x,
        y,
    })(input)
}

fn robot(input: &str) -> IResult<&str, Robot> {
    map(
        separated_pair(
            preceded(tag("p="), coords),
            space1,
            preceded(tag("v="), coords),
        ),
        |(p, v)| Robot { p, v },
    )(input)
}

fn move_robot(r: &Robot, t: i32, (w, h): (i32, i32)) -> Coordinates {
    Coordinates {
        x: (r.p.x + r.v.x * t).rem_euclid(w),
        y: (r.p.y + r.v.y * t).rem_euclid(h),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input)?;
    let (_, robots) =
        separated_list1(newline, robot)(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let (w, h) = (101, 103);
    //let (w, h) = (11, 7);
    let t = 100;
    let final_pos = robots
        .iter()
        .map(|r| move_robot(r, t, (w, h)))
        .collect_vec();
    let mut quads = BTreeMap::<_, usize>::new();
    let _ = final_pos
        .iter()
        .map(|p| {
            *quads
                .entry((p.x.cmp(&(w / 2)), p.y.cmp(&(h / 2))))
                .or_default() += 1
        })
        .collect::<Vec<_>>();

    let answer: usize = quads
        .iter()
        .filter_map(|((h, v), n)| match (h, v) {
            (_, Equal) | (Equal, _) => None,
            _ => Some(n),
        })
        .product();
    println!("Safety factor: {:?}", answer);

    for t in 0..10000 {
        let uniq = robots
            .iter()
            .map(|r| move_robot(r, t, (w, h)))
            .unique()
            .collect::<BTreeSet<_>>();
        let plausible = uniq.len() == robots.len();
        if !plausible {
            continue;
        }

        let out = (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| {
                        if uniq.contains(&Coordinates { x, y }) {
                            '*'
                        } else {
                            '.'
                        }
                    })
                    .join("")
            })
            .join("\n");

        println!("Positions at t = {t}");
        println!("{out}\n");
    }

    return Ok(());
}
