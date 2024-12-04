use regex::Regex;
use std::error::Error;
use std::io::prelude::*;
use std::iter::zip;

use std::collections::HashMap;

fn parse_line(s: &str) -> Option<(i32, i32)> {
    let r = Regex::new(r"^(\d+)\s+(\d+)$").ok()?;
    let caps = r.captures(s)?;
    let a = caps.get(1)?.as_str().parse().ok();
    let b = caps.get(2)?.as_str().parse().ok();
    a.zip(b)
}

fn sorted_unstable<T: Clone + Ord>(vec: &Vec<T>) -> Vec<T> {
    let mut out = vec.to_vec();
    out.sort_unstable();
    out
}

fn distance(in_a: &Vec<i32>, in_b: &Vec<i32>) -> i32 {
    let va = sorted_unstable(in_a);
    let vb = sorted_unstable(in_b);

    let mut distance = 0;
    for (a, b) in zip(va, vb) {
        distance += (a - b).abs();
    }
    distance
}

fn similarity_score(in_a: &Vec<i32>, in_b: &Vec<i32>) -> i32 {
    let mut counts = HashMap::<i32, i32>::new();
    let mut score = 0;
    for b in in_b {
        counts.insert(*b, counts.get(b).copied().unwrap_or(0) + 1);
    }
    for a in in_a {
        score += a * counts.get(a).copied().unwrap_or(0);
    }
    score
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut va: Vec<i32> = Vec::new();
    let mut vb: Vec<i32> = Vec::new();

    for line in stdin.lines() {
        let line = line?;
        parse_line(&line)
            .map(|(a, b)| {
                va.push(a);
                vb.push(b);
            })
            .ok_or(format!("Failed to parse line: {}", line))?;
    }

    let dist = distance(&va, &vb);
    println!("Distance: {}", dist);

    let sim_score = similarity_score(&va, &vb);
    println!("Similarity score: {}", sim_score);

    return Ok(());
}
