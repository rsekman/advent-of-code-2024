use regex::Regex;
use std::error::Error;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut total = 0;
    let mut total_count = 0;
    let mut only_enabled = 0;
    let mut enabled_count = 0;
    let mut enabled = true;

    let mul_pattern = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))")?;
    for line in stdin.lines() {
        for cap in mul_pattern.captures_iter(&line?) {
            let s = cap.get(0).unwrap().as_str();
            if s == "do()" {
                enabled = true;
            } else if s == "don't()" {
                enabled = false;
            } else if s.starts_with("mul(") {
                let x: i32 = cap.get(2).unwrap().as_str().parse()?;
                let y: i32 = cap.get(3).unwrap().as_str().parse()?;
                total += x * y;
                if enabled {
                    only_enabled += x * y;
                    enabled_count += 1;
                }
                total_count += 1;
            }
        }
    }

    println!(
        "Result of {} multiplications summed: {}",
        total_count, total
    );
    println!(
        "Result of {} enabled multiplications summed: {}",
        enabled_count, only_enabled
    );
    return Ok(());
}
