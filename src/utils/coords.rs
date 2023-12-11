use std::{collections::HashSet, hash::Hash};

use num::{CheckedAdd, CheckedSub, Integer, Num, Signed};

use super::direction::Direction;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coordinates<T>
where
    T: Num,
{
    x: T,
    y: T,
}

impl<T> Coordinates<T>
where
    T: Num + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(num::zero(), num::zero())
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}

impl<T> Coordinates<T>
where
    T: Integer + Signed + Copy,
{
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - num::one(),
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + num::one(),
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - num::one(),
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + num::one(),
            y: self.y,
        }
    }
}

impl<T> Coordinates<T>
where
    T: Integer + CheckedAdd + CheckedSub + Copy,
{
    pub fn try_up(&self) -> Option<Self> {
        self.y
            .checked_add(&num::one())
            .map(|y| Self { x: self.x, y })
    }
    pub fn try_down(&self) -> Option<Self> {
        self.y
            .checked_sub(&num::one())
            .map(|y| Self { x: self.x, y })
    }
    pub fn try_right(&self) -> Option<Self> {
        self.x
            .checked_add(&num::one())
            .map(|x| Self { x, y: self.y })
    }
    pub fn try_left(&self) -> Option<Self> {
        self.x
            .checked_sub(&num::one())
            .map(|x| Self { x, y: self.y })
    }
}

impl<T> Coordinates<T>
where
    T: Integer + Hash + CheckedAdd + CheckedSub + Copy,
{
    pub fn orthogonal_neighbors(&self) -> HashSet<Self> {
        let mut neighbors = HashSet::new();
        if let Some(up) = self.try_up() {
            neighbors.insert(up);
        }
        if let Some(down) = self.try_down() {
            neighbors.insert(down);
        }
        if let Some(left) = self.try_left() {
            neighbors.insert(left);
        }
        if let Some(right) = self.try_right() {
            neighbors.insert(right);
        }

        neighbors
    }

    pub fn diagonal_neighbors(&self) -> HashSet<Self> {
        let mut neighbors = HashSet::new();
        if let Some(up_left) = self.try_up().and_then(|up| up.try_left()) {
            neighbors.insert(up_left);
        }
        if let Some(down_left) = self.try_down().and_then(|down| down.try_left()) {
            neighbors.insert(down_left);
        }
        if let Some(up_right) = self.try_up().and_then(|up| up.try_right()) {
            neighbors.insert(up_right);
        }
        if let Some(down_right) = self.try_down().and_then(|down| down.try_right()) {
            neighbors.insert(down_right);
        }

        neighbors
    }

    pub fn all_neighbors(&self) -> HashSet<Self> {
        let mut neighbors = self.diagonal_neighbors();
        neighbors.extend(self.orthogonal_neighbors());
        neighbors
    }
}

impl<T> Coordinates<T>
where
    T: Integer + Signed + Copy,
{
    pub fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }
}

impl<T> Coordinates<T>
where
    T: Integer + Signed + Copy,
{
    pub fn orthogonal_distance(&self, other: Self) -> T {
        let x = if self.x <= other.x {
            num::abs_sub(other.x, self.x)
        } else {
            num::abs_sub(self.x, other.x)
        };
        let y = if self.y <= other.y {
            num::abs_sub(other.y, self.y)
        } else {
            num::abs_sub(self.y, other.y)
        };
        x + y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn orthogonal_distance() {
    //     let coord1: Coordinates<i32> = Coordinates::new(1, 5);
    //     let coord2: Coordinates<i32> = Coordinates::new(4, 9);
    //
    //     assert_eq!(7, coord1.orthogonal_distance(coord2));
    // }
}
