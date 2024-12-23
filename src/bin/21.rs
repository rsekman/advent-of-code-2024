use std::collections::BTreeMap;
use std::error::Error;
use std::io::prelude::*;
use std::iter::once;

use coordinates::two_dimensional::Vector2;
use itertools::{chain, iproduct, repeat_n, Itertools};

//use rayon::prelude::*;

use nom::{
    branch::alt,
    character::complete::{char, u64},
    combinator::{peek, value},
    multi::many1,
    sequence::terminated,
    IResult, Parser,
};

type Coordinates = Vector2<i8>;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum NumericKeypad {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

trait Keyboard: Sized {
    fn coordinates(&self) -> Coordinates;
    fn from_coordinates(c: Coordinates) -> Option<Self>;
}

impl Keyboard for NumericKeypad {
    fn coordinates(&self) -> Coordinates {
        use NumericKeypad::*;
        match self {
            A => (2, 0),
            Zero => (1, 0),
            One => (0, 1),
            Two => (1, 1),
            Three => (2, 1),
            Four => (0, 2),
            Five => (1, 2),
            Six => (2, 2),
            Seven => (0, 3),
            Eight => (1, 3),
            Nine => (2, 3),
        }
        .into()
    }
    fn from_coordinates(Coordinates { x, y }: Coordinates) -> Option<Self> {
        use NumericKeypad::*;
        match (x, y) {
            (2, 0) => Some(A),
            (1, 0) => Some(Zero),
            (0, 1) => Some(One),
            (1, 1) => Some(Two),
            (2, 1) => Some(Three),
            (0, 2) => Some(Four),
            (1, 2) => Some(Five),
            (2, 2) => Some(Six),
            (0, 3) => Some(Seven),
            (1, 3) => Some(Eight),
            (2, 3) => Some(Nine),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum DirectionalKeypad {
    A,
    Up,
    Left,
    Down,
    Right,
}

impl Keyboard for DirectionalKeypad {
    fn coordinates(&self) -> Coordinates {
        use DirectionalKeypad::*;
        (match self {
            Up => (1, 1),
            A => (2, 1),
            Left => (0, 0),
            Down => (1, 0),
            Right => (2, 0),
        })
        .into()
    }
    fn from_coordinates(Coordinates { x, y }: Coordinates) -> Option<Self> {
        use DirectionalKeypad::*;
        match (x, y) {
            (1, 1) => Some(Up),
            (2, 1) => Some(A),
            (0, 0) => Some(Left),
            (1, 0) => Some(Down),
            (2, 0) => Some(Right),
            _ => None,
        }
    }
}

fn paths<T: Keyboard>(a: &T, b: &T) -> Vec<Vec<Coordinates>> {
    let Coordinates { x: dx, y: dy } = b.coordinates() - a.coordinates();
    let xs = if dx >= 0 {
        repeat_n(Coordinates { x: 1, y: 0 }, dx as usize)
    } else {
        repeat_n(Coordinates { x: -1, y: 0 }, dx.abs() as usize)
    };
    let ys = if dy >= 0 {
        repeat_n(Coordinates { x: 0, y: 1 }, dy as usize)
    } else {
        repeat_n(Coordinates { x: 0, y: -1 }, dy.abs() as usize)
    };
    chain!(xs, ys)
        .permutations((dx.abs() + dy.abs()) as usize)
        .filter(|p| {
            p.iter()
                .scan(a.coordinates(), |acc, x| {
                    *acc = *acc + *x;
                    Some(*acc)
                })
                .all(|c| T::from_coordinates(c).is_some())
        })
        .unique()
        .collect()
}

fn dir_path_to_str(p: &Vec<DirectionalKeypad>) -> String {
    use DirectionalKeypad::*;
    p.iter()
        .map(|q| match q {
            Up => "^",
            Down => "v",
            Right => ">",
            Left => "<",
            A => "A",
        })
        .join("")
}

fn path_to_dir(p: &Vec<Coordinates>) -> Vec<DirectionalKeypad> {
    use DirectionalKeypad::*;
    chain![
        p.iter().map(|Coordinates { x, y }| match (x, y) {
            (0, 1) => Up,
            (0, -1) => Down,
            (1, 0) => Right,
            (-1, 0) => Left,
            _ => panic!("Invalid path"),
        }),
        once(A)
    ]
    .collect()
}

fn numeric_to_directional(
    s: &Vec<NumericKeypad>,
    num_paths: &BTreeMap<(NumericKeypad, NumericKeypad), Vec<Vec<Coordinates>>>,
) -> Vec<Vec<DirectionalKeypad>> {
    chain![once(&NumericKeypad::A), s.iter()]
        .tuple_windows()
        .map(|(a, b)| num_paths.get(&(*a, *b)).unwrap().iter())
        .multi_cartesian_product()
        .map(|ps| {
            ps.iter()
                .map(|p| path_to_dir(p))
                .fold(Vec::new(), |mut v, p| {
                    v.extend(p.iter());
                    //v.push(DirectionalKeypad::A);
                    v
                })
        })
        .collect_vec()
}
fn directional_to_directional(
    s: &Vec<DirectionalKeypad>,
    dir_paths: &BTreeMap<(DirectionalKeypad, DirectionalKeypad), Vec<Vec<DirectionalKeypad>>>,
    depth: usize,
    cache: &mut BTreeMap<(Vec<DirectionalKeypad>, usize), usize>,
) -> usize {
    if let Some(&cached) = cache.get(&(s.clone(), depth)) {
        return cached;
    }
    let res = chain![once(&DirectionalKeypad::A), s.iter()]
        .tuple_windows()
        .map(|(a, b)| {
            let paths = dir_paths.get(&(*a, *b)).unwrap();
            if depth == 0 {
                return s.len();
            } else if depth == 1 {
                paths.iter().map(Vec::len).min().unwrap()
            } else {
                paths
                    .into_iter()
                    .map(|p| directional_to_directional(p, dir_paths, depth - 1, cache))
                    .min()
                    .unwrap()
            }
        })
        .sum::<usize>();
    cache.insert((s.clone(), depth), res);
    res
}

fn parse_numeric_kbd(input: &str) -> IResult<&str, (Vec<NumericKeypad>, u64)> {
    use NumericKeypad::*;
    peek(many1(alt((
        value(A, char('A')),
        value(Zero, char('0')),
        value(One, char('1')),
        value(Two, char('2')),
        value(Three, char('3')),
        value(Four, char('4')),
        value(Five, char('5')),
        value(Six, char('6')),
        value(Seven, char('7')),
        value(Eight, char('8')),
        value(Nine, char('9')),
    ))))
    .and(terminated(u64, char('A')))
    .parse(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Precompute all shortest paths on both types of keyboards
    let num_keys = {
        use NumericKeypad::*;
        vec![
            A, Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine,
        ]
    };
    let num_paths = BTreeMap::from_iter(
        iproduct![num_keys.iter(), num_keys.iter()].map(|(a, b)| ((*a, *b), paths(a, b))),
    );
    let dir_keys = {
        use DirectionalKeypad::*;
        vec![A, Up, Left, Down, Right]
    };
    let dir_paths = BTreeMap::from_iter(
        iproduct![dir_keys.iter(), dir_keys.iter()]
            .map(|(a, b)| ((*a, *b), paths(a, b).iter().map(path_to_dir).collect())),
    );

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut cache = BTreeMap::new();

    let n_directionals = 3;
    let mut total_complexity = 0;

    let n_directionals_long = 26;
    let mut total_complexity_long = 0;
    for line in stdin.lines() {
        let (_, (n_sequence, prefix)) =
            parse_numeric_kbd(&line?).map_err(|e| format!("Invalid input: {e}"))?;
        let dir_sequences = numeric_to_directional(&n_sequence, &num_paths);

        let shortest = dir_sequences
            .iter()
            // minus one because one of the directional keyboard iterations was already taken care
            // of transforming from the numerical keyboard
            .map(|s| directional_to_directional(s, &dir_paths, n_directionals - 1, &mut cache))
            .min()
            .unwrap();
        let complexity = shortest * (prefix as usize);
        total_complexity += complexity;

        let shortest_long = dir_sequences
            .iter()
            .map(|s| directional_to_directional(s, &dir_paths, n_directionals_long - 1, &mut cache))
            .min()
            .unwrap();
        let complexity = shortest_long * (prefix as usize);
        total_complexity_long += complexity;
    }

    println!("Total complexity of the five codes ({n_directionals} robots): {total_complexity}");
    println!(
        "Total complexity of the five codes ({n_directionals_long} robots): {total_complexity_long}"
    );
    return Ok(());
}
