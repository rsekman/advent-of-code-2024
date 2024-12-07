use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list1,
    sequence::separated_pair, IResult,
};
use std::error::Error;
use std::io::prelude::*;

enum Operation {
    Add,
    Mul,
    Concat,
}

fn concat_numbers(x: u64, y: u64) -> u64 {
    x * 10u64.pow((y as f64).log10().floor() as u32 + 1) + y
}

fn calibration_is_valid(target: u64, inputs: &Vec<u64>, ops: &Vec<&Operation>) -> bool {
    let mut inputs = inputs.iter();
    let Some(acc) = inputs.next() else {
        return false;
    };
    let mut acc: u64 = *acc;
    for (&v, op) in inputs.zip(ops) {
        match op {
            Operation::Add => acc += v,
            Operation::Mul => acc *= v,
            Operation::Concat => acc = concat_numbers(acc, v),
        }
        if acc > target {
            break;
        }
    }
    acc == target
}

fn calibration_can_be_valid(target: u64, inputs: &Vec<u64>, allowed_ops: &Vec<Operation>) -> bool
where
{
    itertools::repeat_n(allowed_ops.iter(), inputs.len() - 1)
        .multi_cartesian_product()
        .map(|ops| calibration_is_valid(target, &inputs, &ops))
        .any(|x| x)
}

fn parse_calibration(s: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(
        u64::<&str, nom::error::Error<&str>>,
        tag(": "),
        separated_list1(tag(" "), u64),
    )(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut total = 0;
    let mut total_concat = 0;
    let addmul = vec![Operation::Add, Operation::Mul];
    let addmulconcat = vec![Operation::Add, Operation::Mul, Operation::Concat];
    for line in stdin.lines() {
        let line = line?;
        let (_, (target, inputs)) = parse_calibration(&line).map_err(|e| e.to_owned())?;
        if calibration_can_be_valid(target, &inputs, &addmul) {
            total += target;
            total_concat += target;
        } else if calibration_can_be_valid(target, &inputs, &addmulconcat) {
            total_concat += target;
        }
    }
    println!("Total calibration result (Add, Mul): {}", total);
    println!(
        "Total calibration result (Add, Mul, Concat): {}",
        total_concat
    );

    return Ok(());
}
