#![feature(map_try_insert)]
use std::error::Error;
use std::io::prelude::*;

use itertools::Itertools;
use rayon::prelude::*;
use std::collections::BTreeMap;

type DiffSeq = (i64, i64, i64, i64);

fn mix_and_prune(x: u64, y: u64) -> u64 {
    (x ^ y) % 16777216
}

fn secret(x: u64) -> u64 {
    let y = mix_and_prune(x * 64, x);
    let z = mix_and_prune(y / 32, y);
    mix_and_prune(z * 2048, z)
}

fn secrets(seed: u64, n_folds: usize) -> impl Iterator<Item = u64> {
    (0..n_folds).scan(seed, |state, _| {
        let prev = *state;
        *state = secret(*state);
        Some(prev % 10)
    })
}

fn buys(secrets: &Vec<u64>) -> BTreeMap<DiffSeq, u64> {
    let diffs = secrets
        .iter()
        .tuple_windows()
        .map(|(y, x)| x.wrapping_sub(*y) as i64)
        .tuple_windows::<DiffSeq>();
    let prices = Iterator::zip(diffs, secrets.iter().skip(4));
    let mut map = BTreeMap::new();
    for (k, v) in prices {
        let _ = map.try_insert(k, *v);
    }
    map
}

fn sum_by_key(prices: &Vec<BTreeMap<DiffSeq, u64>>) -> BTreeMap<DiffSeq, u64> {
    let mut out = BTreeMap::new();
    for ps in prices {
        for (&k, &v) in ps {
            let r = out.entry(k).or_insert(0);
            *r += v;
        }
    }
    out
}

fn maximize_buys(seeds: Vec<u64>, n_folds: usize) -> (u64, Option<DiffSeq>) {
    let secrets: Vec<Vec<u64>> = seeds
        .par_iter()
        .map(|&s| secrets(s, n_folds).collect_vec())
        .collect();

    let m = secrets.par_iter().map(buys).collect();
    let summed = sum_by_key(&m);
    let m = summed.iter().max_by_key(|(_, v)| *v);
    match m {
        Some((seq, v)) => (*v, Some(*seq)),
        None => (0, None),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let n_folds = 2000;

    let seeds: Vec<u64> = stdin
        .lines()
        .try_collect::<_, Vec<_>, _>()?
        .iter()
        .map(|s| s.parse())
        .try_collect()?;
    let answer: u64 = seeds
        .iter()
        .map(|x| (0..n_folds).fold(*x, |s, _| secret(s)))
        .sum();

    println!("Sum of {n_folds}th secret numbers: {answer}.");

    let (max_buys, seq) = maximize_buys(seeds, n_folds);
    let seq = seq.ok_or("No maximizing sequence found (empty input?)")?;
    println!("To maximize buys, use: {seq:?} ({max_buys} bananas)");

    return Ok(());
}
