use std::error::Error;
use std::io::prelude::*;

use itertools::Itertools;

use std::collections::{BTreeMap, BTreeSet};

type PebbleValue = u64;
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
struct Pebble {
    value: PebbleValue,
    children: PebbleChildren,
}
type PebbleChildren = (Option<PebbleValue>, Option<PebbleValue>);

fn blink(v: PebbleValue, seen: &mut BTreeMap<PebbleValue, PebbleChildren>) -> PebbleChildren {
    let (left, right) = if v == 0 {
        (Some(1), None)
    } else if v.ilog10() % 2 == 1 {
        let (l, r) = split_pebble(v);
        (Some(l), Some(r))
    } else {
        (Some(2024 * v), None)
    };
    seen.entry(v).or_insert((left, right));
    (left, right)
}

fn split_pebble(p: PebbleValue) -> (PebbleValue, PebbleValue) {
    let n = 10u64.pow((p.ilog10() + 1) / 2);
    (p / n, p % n)
}

fn n_descendants(
    p: PebbleValue,
    max_depth: usize,
    mut seen: &mut BTreeMap<PebbleValue, PebbleChildren>,
    cache: &mut BTreeMap<(PebbleValue, usize), usize>,
) -> usize {
    do_n_descendants(p, 0, max_depth, seen, cache)
}

fn do_n_descendants(
    v: PebbleValue,
    depth: usize,
    max_depth: usize,
    mut seen: &mut BTreeMap<PebbleValue, PebbleChildren>,
    mut cache: &mut BTreeMap<(PebbleValue, usize), usize>,
) -> usize {
    let out = if depth < max_depth {
        let (left, right) = blink(v, seen);
        let left_descendants = if let Some(x) = left {
            cache
                .get(&(x, max_depth - depth - 1))
                .map(|r| *r)
                .unwrap_or_else(|| do_n_descendants(x, depth + 1, max_depth, &mut seen, &mut cache))
        } else {
            0
        };
        let right_descendants = if let Some(x) = right {
            cache
                .get(&(x, max_depth - depth - 1))
                .map(|r| *r)
                .unwrap_or_else(|| do_n_descendants(x, depth + 1, max_depth, &mut seen, &mut cache))
        } else {
            0
        };
        left_descendants + right_descendants
    } else {
        1
    };
    cache.insert((v, max_depth - depth), out);
    out
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let line = stdin.lines().next().ok_or("No input!")??;

    let ps: Vec<PebbleValue> = line
        .split_whitespace()
        .map(&str::parse::<PebbleValue>)
        .collect::<Result<Vec<PebbleValue>, _>>()?;

    let mut seen = BTreeMap::<PebbleValue, PebbleChildren>::new();
    let mut cache = BTreeMap::<(PebbleValue, usize), usize>::new();

    let short = 25;
    let descs_short: usize = ps
        .iter()
        .map(|p| n_descendants(*p, short, &mut seen, &mut cache))
        .sum();
    println!(
        "After {short} blinks: {descs_short} ({} unique values)",
        seen.len()
    );

    let long = 75;
    let descs_long: usize = ps
        .iter()
        .map(|p| n_descendants(*p, long, &mut seen, &mut cache))
        .sum();
    println!(
        "After {long} blinks: {descs_long} ({} unique values)",
        seen.len()
    );

    return Ok(());
}
