use std::cmp::Ordering;
use std::error::Error;
use std::io::prelude::*;

fn within_tolerance(prev: i32, new: i32) -> bool {
    let d = (prev - new).abs();
    1 <= d && d <= 3
}

fn is_safe(log: &Vec<i32>, dampener: bool) -> bool {
    let mut ord: Option<Ordering> = None;
    let mut prev: Option<i32> = None;

    for (i, cur) in log.into_iter().enumerate() {
        let tolerated = prev.map(|x| within_tolerance(x, *cur)).unwrap_or(true);
        let correct_order = prev.zip(ord).map(|(p, o)| cur.cmp(&p) == o).unwrap_or(true);
        if !(tolerated && correct_order) {
            if !dampener {
                return false;
            } else {
                for k in 0..(i + 1) {
                    /* Three values are involved in accepting the current level: the current level,
                     * the previous level, and the level before that. This is because we need
                     *     sign(x-y) == sign(y-z).
                     * If the current level is rejected, we retry, in turn, with each of these three values
                     * dropped.
                     */
                    let mut dropped = Vec::new();
                    dropped.append(&mut log[..i - k].to_vec());
                    dropped.append(&mut log[i - k + 1..].to_vec());
                    if is_safe(&dropped, false) {
                        return true;
                    }
                }
                return false;
            }
        }
        ord = ord.or(prev.map(|x| cur.cmp(&x)));
        prev = Some(*cur);
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut safe: u32 = 0;
    let mut safe_with_dampener: u32 = 0;
    for line in stdin.lines() {
        let line = line?;
        let mut report: Vec<i32> = Vec::new();
        for entry in line.split_whitespace() {
            report.push(entry.parse()?);
        }
        if is_safe(&report, false) {
            safe += 1;
        }
        if is_safe(&report, true) {
            safe_with_dampener += 1;
        }
    }

    println!("Number of safe reports: {}", safe);
    println!(
        "Number of safe reports with dampener: {}",
        safe_with_dampener
    );
    return Ok(());
}
