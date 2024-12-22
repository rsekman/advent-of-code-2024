use std::error::Error;
use std::io::prelude::*;

use itertools::Itertools;
use rayon::prelude::*;

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

fn find_buy(secrets: &Vec<u64>, diffs: &Vec<DiffSeq>, seq: &DiffSeq) -> u64 {
    if let Some(idx) = diffs.iter().position(|s| s == seq) {
        secrets[idx + 4]
    } else {
        return 0;
    }
}

fn total_buys(secrets: &Vec<Vec<u64>>, diffs: &Vec<Vec<DiffSeq>>, seq: &DiffSeq) -> u64 {
    Iterator::zip(secrets.iter(), diffs.iter())
        .map(|(ss, ds)| find_buy(ss, ds, seq))
        .sum()
}

fn maximize_buys(seeds: Vec<u64>, n_folds: usize) -> (u64, Option<DiffSeq>) {
    let secrets = seeds
        .iter()
        .map(|&s| secrets(s, n_folds).collect_vec())
        .collect_vec();

    let diffs = secrets
        .iter()
        .map(|ss| {
            ss.iter()
                .tuple_windows()
                .map(|(y, x)| x.wrapping_sub(*y) as i64)
                .tuple_windows::<DiffSeq>()
                .collect_vec()
        })
        .collect_vec();

    let seqs = itertools::repeat_n(-9..=9, 4)
        .multi_cartesian_product()
        .map(|v| v.iter().cloned().tuple_windows::<DiffSeq>().next().unwrap())
        .collect_vec();

    println!("Finding maximum sequence...");
    let m = seqs
        .par_iter()
        .max_by_key(|seq| total_buys(&secrets, &diffs, seq));
    match m {
        Some(seq) => (total_buys(&secrets, &diffs, seq), Some(*seq)),
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
