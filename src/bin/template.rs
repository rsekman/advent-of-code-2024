use std::error::Error;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut answer = 0;
    for line in stdin.lines() {
        let line = line?;
        // BODY
    }
    println!("The answer is: {}", answer);

    return Ok(());
}
