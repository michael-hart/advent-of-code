use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let left = Point::new(1, 2);
        let right = Point::new(5, 6);
        let expected = Point::new(6, 8);
        assert_eq!(left + right, expected);
    }

    #[test]
    fn test_add_assign() {
        let mut result = Point::new(1, 2);
        result += Point::new(5, 6);
        assert_eq!(result, Point::new(6, 8));
    }
}