#![feature(let_chains)]
use std::error::Error;
use std::io::prelude::*;

use itertools::Itertools;
use nom::{
    character::complete::{alpha1, char, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_lan(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(newline, separated_pair(alpha1, char('-'), alpha1))(input)
}

fn cliques<'a>(
    c: &'a str,
    lan: &BTreeMap<&'a str, BTreeSet<&'a str>>,
    depth: usize,
    cache: &mut BTreeMap<(&'a str, usize), BTreeSet<BTreeSet<&'a str>>>,
) -> BTreeSet<BTreeSet<&'a str>> {
    if let Some(cached) = cache.get(&(c, depth)) {
        return cached.clone();
    }
    if depth == 0 {
        let out = BTreeSet::from_iter(vec![BTreeSet::from_iter(std::iter::once(c))]);
        cache.insert((c, depth), out.clone());
        out
    } else {
        let mut out = BTreeSet::new();
        for n in lan.get(c).unwrap() {
            let prev = cliques(n, &lan, depth - 1, cache);
            for cli in prev {
                if cli.iter().all(|m| lan.get(m).unwrap().contains(c)) {
                    let mut new = cli.clone();
                    new.insert(c);
                    out.insert(new);
                }
            }
        }
        cache.insert((c, depth), out.clone());
        out
    }
}

use std::collections::{BTreeMap, BTreeSet};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    let mut lan: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    let mut computers: BTreeSet<&str> = BTreeSet::new();
    stdin.read_to_string(&mut input)?;
    let (_, parsed) = parse_lan(&input).map_err(|e| format!("Invalid input: {e}"))?;
    for (l, r) in parsed {
        lan.entry(l).or_insert(BTreeSet::new()).insert(r);
        computers.insert(l);
        lan.entry(r).or_insert(BTreeSet::new()).insert(l);
        computers.insert(r);
    }

    let mut cache = BTreeMap::new();

    let identifier = 't';
    let n_cliques = computers
        .iter()
        .filter(|c| c.starts_with(identifier))
        .flat_map(|c| cliques(c, &lan, 2, &mut cache))
        .collect::<BTreeSet<_>>();
    println!(
        "Number of 3-cliques with at least one computer beginning with {identifier}: {}",
        n_cliques.len()
    );

    let mut largest_clique = BTreeSet::<&str>::new();
    for c in computers.iter() {
        let max_depth = lan.get(c).unwrap().len();
        let largest = (0..max_depth)
            .flat_map(|d| cliques(c, &lan, d, &mut cache))
            .max_by_key(|cl| cl.len());
        if let Some(lrg) = largest
            && lrg.len() > largest_clique.len()
        {
            largest_clique = lrg.clone();
        }
    }
    println!("LAN password: {}", largest_clique.iter().join(","));
    return Ok(());
}
