use std::cmp::Ordering;
use std::error::Error;
use std::io::prelude::*;

fn within_tolerance(prev: i32, new: i32) -> bool {
    let d = (prev - new).abs();
    1 <= d && d <= 3
}

fn is_safe(log: &Vec<i32>) -> bool {
    let mut ord: Option<Ordering> = None;
    let mut prev: Option<i32> = None;

    for new in log {
        let tolerated = prev.map(|x| within_tolerance(x, *new)).unwrap_or(true);
        let correct_order = prev
            .zip(ord)
            .map(|(p, o)| new.cmp(&p) == o)
            .unwrap_or(true);
        if !(tolerated && correct_order) {
            return false;
        }
        ord = ord.or(prev.map(|x| new.cmp(&x)));
        prev = Some(*new);
    }
    true
}

// Ugly O(n^2) solution, probably not optimal
fn is_safe_damped(log: &Vec<i32>) -> bool {
    for n in 0..log.len() {
        let mut head = log[..n].to_vec();
        let mut tail = log[n + 1..].to_vec();
        head.append(&mut tail);
        if is_safe(&head) {
            return true;
        }
    }
    false
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
        if is_safe(&report) {
            safe += 1;
        }
        if is_safe_damped(&report) {
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
