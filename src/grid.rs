use coordinates::two_dimensional::Vector2;
use num::traits::{CheckedAdd, CheckedSub};

pub type UPoint = Vector2<usize>;
pub type IPoint = Vector2<isize>;

pub fn neighbors_within_bounds(p: &UPoint, (w, h): (usize, usize)) -> Vec<UPoint> {
    vec![
        p.checked_add(&(1, 0).into()),
        p.checked_sub(&(1, 0).into()),
        p.checked_add(&(0, 1).into()),
        p.checked_sub(&(0, 1).into()),
    ]
    .iter()
    .filter_map(|q| *q)
    .filter(|q| q.x <= w && q.y <= h)
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
