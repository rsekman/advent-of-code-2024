use coordinates::two_dimensional::Vector2;
use num::traits::{CheckedAdd, CheckedSub};

pub type UPoint = Vector2<usize>;
pub type IPoint = Vector2<isize>;

#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

pub fn clockwise(c: CardinalDirection) -> CardinalDirection {
    match c {
        CardinalDirection::North => CardinalDirection::East,
        CardinalDirection::East => CardinalDirection::South,
        CardinalDirection::South => CardinalDirection::West,
        CardinalDirection::West => CardinalDirection::North,
    }
}

pub fn counterclockwise(c: CardinalDirection) -> CardinalDirection {
    match c {
        CardinalDirection::North => CardinalDirection::West,
        CardinalDirection::East => CardinalDirection::North,
        CardinalDirection::South => CardinalDirection::East,
        CardinalDirection::West => CardinalDirection::South,
    }
}

pub fn step(p: UPoint, c: CardinalDirection) -> Option<UPoint> {
    match c {
        CardinalDirection::North => p.checked_sub(&(0, 1).into()),
        CardinalDirection::South => p.checked_add(&(0, 1).into()),
        CardinalDirection::East => p.checked_add(&(1, 0).into()),
        CardinalDirection::West => p.checked_sub(&(1, 0).into()),
    }
}

impl std::ops::Neg for CardinalDirection {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::East,
        }
    }
}

pub fn neighbors_within_bounds(p: &UPoint, (w, h): (usize, usize)) -> Vec<UPoint> {
    neighbors_unbounded(&p)
        .iter()
        .cloned()
        .filter(|q| q.x <= w && q.y <= h)
        .collect()
}

pub fn neighbors_unbounded(p: &UPoint) -> Vec<UPoint> {
    vec![
        p.checked_add(&(1, 0).into()),
        p.checked_sub(&(1, 0).into()),
        p.checked_add(&(0, 1).into()),
        p.checked_sub(&(0, 1).into()),
    ]
    .iter()
    .filter_map(|q| *q)
    .collect()
}

pub fn neighbors(p: IPoint) -> Vec<IPoint> {
    vec![
        p + (1isize, 0isize).into(),
        p + (-1isize, 0isize).into(),
        p + (0isize, 1isize).into(),
        p + (0isize, -1isize).into(),
    ]
}
