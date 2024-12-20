use std::error::Error;
use std::io::prelude::*;

use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

use itertools::Itertools;
use regex::Regex;

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(
        terminated(separated_list1(tag(", "), alpha1), newline),
        newline,
        separated_list1(newline, alpha1),
    )(input)
}

fn do_possible_arrangements<'a>(
    design: &'a str,
    towels: &Vec<&str>,
    cache: &mut BTreeMap<&'a str, usize>,
) -> usize {
    if design.len() == 0 {
        return 1;
    }
    if let Some(c) = cache.get(design) {
        return *c;
    }
    let mut out = 0;
    for t in towels {
        if design.starts_with(t) {
            out += do_possible_arrangements(&design[t.len()..], towels, cache);
        }
    }
    cache.insert(design, out);
    out
}

fn possible_arrangements(design: &str, towels: &Vec<&str>) -> usize {
    do_possible_arrangements(design, towels, &mut BTreeMap::new())
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, (towels, displays)) = parse_input(&input).map_err(|e| format!("Invalid input: {e}"))?;

    {
        let rstr = format!("^({})*$", towels.iter().join("|"));
        let re = Regex::new(&rstr)?;

        let possible_count = displays.iter().filter(move |d| re.is_match(d)).count();
        println!("There are {possible_count} possible displays");
    }

    let ways = displays
        .iter()
        .map(|d| possible_arrangements(d, &towels))
        .sum::<usize>();
    println!("The sum of distinct ways to arrange the towels is {ways}.");

    return Ok(());
}
