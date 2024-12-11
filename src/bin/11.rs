use std::error::Error;
use std::io::prelude::*;

use itertools::Itertools;
use std::collections::BTreeMap;

type PebbleValue = u64;
fn split_pebble(p: PebbleValue) -> (PebbleValue, PebbleValue) {
    let n = 10u64.pow((p.ilog10() + 1) / 2);
    (p / n, p % n)
}

fn n_descendants(
    p: PebbleValue,
    max_depth: usize,
    cache: &mut BTreeMap<(PebbleValue, usize), usize>,
) -> usize {
    do_n_descendants(p, 0, max_depth, cache)
}

fn do_n_descendants(
    v: PebbleValue,
    depth: usize,
    max_depth: usize,
    mut cache: &mut BTreeMap<(PebbleValue, usize), usize>,
) -> usize {
    let mut get = |x| {
        cache
            .get(&(x, max_depth - depth - 1))
            .map(|r| *r)
            .unwrap_or_else(|| do_n_descendants(x, depth + 1, max_depth, &mut cache))
    };
    let out = if depth < max_depth {
        if v == 0 {
            get(1)
        } else if v.ilog10() % 2 == 1 {
            let (left, right) = split_pebble(v);
            get(left) + get(right)
        } else {
            get(2024 * v)
        }
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

    let mut cache = BTreeMap::<(PebbleValue, usize), usize>::new();

    let short = 25;
    let descs_short: usize = ps
        .iter()
        .map(|p| n_descendants(*p, short, &mut cache))
        .sum();
    println!(
        "After {short} blinks: {descs_short} ({} unique values)",
        cache.keys().sorted().unique_by(|c| c.0).count()
    );

    let long = 75;
    let descs_long: usize = ps.iter().map(|p| n_descendants(*p, long, &mut cache)).sum();
    println!(
        "After {long} blinks: {descs_long} ({} unique values)",
        cache.keys().sorted().unique_by(|c| c.0).count()
    );

    return Ok(());
}
