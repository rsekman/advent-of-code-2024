use std::error::Error;
use std::io::prelude::*;

type Grid = Vec<Vec<char>>;
type Point = (usize, usize);
type Delta = (isize, isize);

fn step((x, y): Point, (dx, dy): Delta) -> Option<Point> {
    x.checked_add_signed(dx).zip(y.checked_add_signed(dy))
}

fn get_in_grid(grid: &Grid, (x, y): Point) -> Option<&char> {
    grid.get(x).and_then(|v| v.get(y))
}

fn search_for_xmas(grid: &Grid, mut p: Point, dp: Delta) -> bool {
    let xmas = "XMAS";
    let mut found = String::from("");
    for _ in xmas.chars() {
        let l = get_in_grid(&grid, p);
        if let Some(x) = l {
            found.push(*x);
        }
        if let Some(next) = step(p, dp) {
            p = next;
        } else {
            break;
        }
    }
    found == xmas
}

fn find_xmas(grid: &Grid, start: Point) -> u32 {
    let directions: Vec<(isize, isize)> = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, 1),
        (-1, -1),
        (1, -1),
    ];
    let mut count = 0;
    for d in directions {
        if search_for_xmas(&grid, start, d) {
            count += 1
        }
    }
    count
}

fn word_along_vec(grid: &Grid, center: Point, vec: Vec<Delta>) -> String {
    let mut word = String::from("");
    for dx in vec {
        match step(center, dx).and_then(|p| get_in_grid(&grid, p)) {
            Some(c) => word.push(*c),
            _ => continue,
        }
    }
    word
}

fn is_mas(w: String) -> bool {
    return w == "MAS" || w == "SAM";
}

fn find_cross_mas(grid: &Grid, center: Point) -> bool {
    let se = vec![(-1, -1), (0, 0), (1, 1)];
    let nw = vec![(-1, 1), (0, 0), (1, -1)];

    let se_word = word_along_vec(&grid, center, se);
    let nw_word = word_along_vec(&grid, center, nw);
    is_mas(se_word) && is_mas(nw_word)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut g: Grid = vec![];
    for line in stdin.lines() {
        let line = line?;
        g.push(vec![]);
        let row = g.last_mut().unwrap();
        for c in line.chars() {
            row.push(c);
        }
    }

    let mut xmas_count = 0;
    let mut cross_mas_count = 0;
    for (x, row) in g.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let n = find_xmas(&g, (x, y));
            xmas_count += n;
            if find_cross_mas(&g, (x, y)) {
                cross_mas_count += 1;
            }
        }
    }
    println!("XMAS count: {}", xmas_count);
    println!("X-MAS count: {}", cross_mas_count);

    return Ok(());
}
