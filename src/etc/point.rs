use std::fmt::Display;
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign, Neg};
use num_traits::PrimInt;
use super::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
    pub const fn unit_up() -> Self {
        Self { x: 0, y: -1 }
    }
    pub const fn unit_down() -> Self {
        Self { x: 0, y: 1 }
    }
    pub const fn unit_left() -> Self {
        Self { x: -1, y: 0 } 
    }
    pub const fn unit_right() -> Self {
        Self { x: 1, y: 0 }
    }
    pub fn up(&self) -> Self {
        self + Self::unit_up()
    }
    pub fn down(&self) -> Self {
        self + Self::unit_down()
    }
    pub fn left(&self) -> Self {
        self + Self::unit_left()
    }
    pub fn right(&self) -> Self {
        self + Self::unit_right()
    }
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            self.up(),
            self.down(),
            self.left(),
            self.right(),
        ]
    }
    pub fn neighbors_in_bounds<'a, T>(&self, grid: &'a Grid<T>) -> impl IntoIterator<Item = Self> + 'a {
        self.neighbors().into_iter().filter(move |p| grid.is_in_bounds(*p))
    }
    pub fn neighbors_diagonal(&self) -> Vec<Self> {
        vec![
            self.up(),
            self.down(),
            self.left(),
            self.right(),
            self.up().left(),
            self.up().right(),
            self.down().left(),
            self.down().right(),
        ]
    }
    pub fn neighbors_diagonal_in_bounds<'a, T>(&self, grid: &'a Grid<T>) -> impl IntoIterator<Item = Self> + 'a {
        self.neighbors_diagonal().into_iter().filter(move |p| grid.is_in_bounds(*p))
    }
}

impl<I: PrimInt + Display> From<(I, I)> for Point {
    fn from((x, y): (I, I)) -> Self {
        let x = x.to_isize().unwrap_or_else(
            || panic!("Value out of range for isize")
        );
        let y = y.to_isize().unwrap_or_else(
            || panic!("Value out of range for isize")
        );
        Self::new(x, y)
    }
}

impl From<Point> for (isize, isize) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, other: Point) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<Point> for &Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<&Point> for Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<&Point> for &Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other;
    }
}

impl Sub<Point> for Point {
    type Output = Self;
    fn sub(self, other: Point) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<Point> for &Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<&Point> for Point {
    type Output = Point;
    fn sub(self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;
    fn sub(self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = *self - other;
    }
}

impl Mul<isize> for Point {
    type Output = Self;
    fn mul(self, scalar: isize) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl Mul<isize> for &Point {
    type Output = Point;
    fn mul(self, scalar: isize) -> Point {
        Point::new(self.x * scalar, self.y * scalar)
    }
}

impl Mul<Point> for isize {
    type Output = Point;
    fn mul(self, point: Point) -> Point {
        Point::new(point.x * self, point.y * self)
    }
}

impl Mul<&Point> for isize {
    type Output = Point;
    fn mul(self, point: &Point) -> Point {
        Point::new(point.x * self, point.y * self)
    }
}

impl MulAssign<isize> for Point {
    fn mul_assign(&mut self, scalar: isize) {
        *self = *self * scalar;
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Neg for &Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point::new(-self.x, -self.y)
    }
}
