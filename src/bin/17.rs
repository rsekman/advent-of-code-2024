use std::error::Error;
use std::io::prelude::*;

use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, oct_digit1, u64, u8},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

use itertools::Itertools;

#[derive(Clone)]
struct CSC {
    a: u64,
    b: u64,
    c: u64,
    tape: Vec<u8>,
    iptr: usize,
    output: Vec<u8>,
}

type CSCResult = Result<bool, ()>;

impl CSC {
    fn literal(&self) -> Result<u64, ()> {
        self.tape.get(self.iptr + 1).ok_or(()).map(|c| *c as u64)
    }

    fn combo(&self) -> Result<u64, ()> {
        let op = *self.tape.get(self.iptr + 1).ok_or(())?;
        match op {
            0 | 1 | 2 | 3 => Ok(op as u64),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            7 => Err(()),
            _ => Err(()),
        }
    }

    fn adv(&mut self) -> CSCResult {
        let v = self.combo()?;
        self.a /= 2u64.pow(v as u32);
        Ok(true)
    }

    fn bdv(&mut self) -> CSCResult {
        let v = self.combo()?;
        self.b = self.a / 2u64.pow(v as u32);
        Ok(true)
    }

    fn cdv(&mut self) -> CSCResult {
        let v = self.combo()?;
        self.c = self.a / 2u64.pow(v as u32);
        Ok(true)
    }

    fn bxl(&mut self) -> CSCResult {
        self.b = self.literal()? ^ self.b;
        Ok(true)
    }

    fn bst(&mut self) -> CSCResult {
        self.b = self.combo()? % 8;
        Ok(true)
    }

    fn jnz(&mut self) -> CSCResult {
        if self.a != 0 {
            self.iptr = self.literal()? as usize;
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn bxc(&mut self) -> CSCResult {
        self.b = self.b ^ self.c;
        Ok(true)
    }

    fn out(&mut self) -> CSCResult {
        self.output.push((self.combo()? % 8) as u8);
        Ok(true)
    }

    fn run(&mut self) -> CSCResult {
        loop {
            let opcode = self.tape.get(self.iptr).ok_or(())?;
            let advance = match opcode {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(),
                7 => self.cdv(),
                _ => Err(()),
            }?;
            if advance {
                self.iptr += 2;
            }
        }
    }
}

fn parse_reg(input: &str) -> IResult<&str, u64> {
    alt((
        (map_res(preceded(char('0'), oct_digit1), |s| {
            u64::from_str_radix(s, 8)
        })),
        u64,
    ))(input)
}

fn parse_csc(input: &str) -> IResult<&str, CSC> {
    map(
        separated_pair(
            tuple((
                delimited(tag("Register A: "), parse_reg, newline),
                delimited(tag("Register B: "), u64, newline),
                delimited(tag("Register C: "), u64, newline),
            )),
            newline,
            preceded(tag("Program: "), separated_list1(char(','), u8)),
        ),
        |((a, b, c), tape)| CSC {
            a,
            b,
            c,
            tape,
            iptr: 0,
            output: Vec::new(),
        },
    )(input)
}

fn find_quine(csc: &CSC) -> Option<u64> {
    for k in 0..8 {
        if let Some(v) = do_brute_quine(csc.clone(), k) {
            return Some(v);
        }
    }
    None
}

fn do_brute_quine(csc: CSC, start: u64) -> Option<u64> {
    if start > 8u64.pow(csc.tape.len() as u32) {
        return None;
    }
    let mut comp = csc.clone();
    comp.a = start;
    let _ = comp.run();
    let l = comp.tape.len();
    let n = comp.output.len();
    if comp.output == comp.tape[l - n..] {
        if n == l {
            return Some(start);
        } else {
            for k in 0..8 {
                if let Some(v) = do_brute_quine(csc.clone(), start * 8 + k) {
                    return Some(v);
                }
            }
            None
        }
    } else {
        None
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, comp) = parse_csc(&input).map_err(|e| format!("Invalid input: {e}"))?;
    let mut c = comp.clone();
    let _ = c.run();
    println!(
        "The output of the chronospatial computer is: {}",
        c.output.iter().map(|&c| c.to_string()).join(",")
    );

    println!("Trying to quine: {:?}", comp.tape);
    if let Some(v) = find_quine(&comp) {
        println!("Quine input: 0{:o} = {}", v, v);
        let mut qc = comp.clone();
        qc.a = v;
        let _ = qc.run();
        println!(
            "The output with the quine input is: {}",
            qc.output.iter().map(|&c| c.to_string()).join(",")
        );
    } else {
        println!("Unable to find a quining input state");
    }

    return Ok(());
}
