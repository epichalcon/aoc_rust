use std::hash::Hash;

use num::{Integer, Num, Signed};

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
            y: self.y + num::one(),
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - num::one(),
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
