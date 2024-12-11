use std::collections::HashSet;
use std::error::Error;
use std::io::prelude::*;

use itertools::repeat_n;
use itertools::Itertools;

fn checksum<'a, T: Iterator<Item = &'a Option<u64>>>(disk: T) -> u64 {
    disk.enumerate().fold(0, |acc, (pos, block)| match block {
        Some(file_id) => acc + (pos as u64) * file_id,
        None => acc,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let line = stdin.lines().next().ok_or("No input!")??;

    let blocks = line
        .chars()
        .map(|c| c.to_digit(10).ok_or("Invalid input!"))
        .try_collect::<_, Vec<_>, _>()?
        .iter()
        .enumerate()
        .map(|(idx, n)| match idx % 2 {
            0 => repeat_n(Some((idx as u64) / 2), *n as usize),
            1 => repeat_n(None, *n as usize),
            _ => unreachable!(),
        })
        .flatten()
        .collect_vec();

    let mut defrag = blocks.clone();
    let mut front = 0;
    let mut back = blocks.len() - 1;
    while back > front {
        while defrag[front].is_some() {
            front += 1
        }
        if defrag[back].is_some() {
            defrag[front] = defrag[back];
            defrag[back] = None;
            front += 1
        }
        back -= 1;
    }
    println!(
        "Checksum after defragmenting by block: {:?}",
        checksum(defrag.iter())
    );

    let mut unoccupied = blocks
        .iter()
        .chunk_by(|&b| b.is_some())
        .into_iter()
        .scan(0, |pos, (occupied, run)| {
            let n = run.count();
            *pos += n;
            if !occupied {
                Some(Some((*pos - n, n)))
            } else {
                Some(None)
            }
        })
        .filter_map(|x| x)
        .collect_vec();

    defrag = blocks.clone();
    let mut back = defrag.len() - 1;
    let mut file_end = back;
    let mut cur_file = defrag[back];
    let mut moved = HashSet::<u64>::new();
    while back > 0 {
        back -= 1;
        if defrag[back] == cur_file {
            continue;
        } else if let Some(f) = cur_file {
            // try to find somewhere to put the current file
            if !moved.contains(&f) {
                let file_size = file_end - back;
                match unoccupied
                    .iter()
                    .find_position(|(pos, n)| *n >= file_size && *pos < back)
                {
                    Some((chunk_idx, (pos, len))) => {
                        for n in 0..file_size {
                            defrag[pos + n] = defrag[file_end - n];
                            defrag[file_end - n] = None;
                        }
                        unoccupied[chunk_idx] = (*pos + file_size, len - file_size);
                    }
                    None => {}
                }
                moved.insert(f);
            }
        }
        cur_file = defrag[back];
        file_end = back;
    }

    println!(
        "Checksum after defragmenting by file:  {}",
        checksum(defrag.iter())
    );

    return Ok(());
}
