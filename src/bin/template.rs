use std::error::Error;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    for line in stdin.lines() {
        let line = line?;

        let mut answer = 0;
        // BODY
        println!("The answer is: {}", answer);
    }

    return Ok(());
}
