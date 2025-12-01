use std::iter::Enumerate;
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

use num_traits::PrimInt;

use super::Point;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

pub struct GridEnumerate<'a, T> {
    iter: Enumerate<Iter<'a, T>>,
    mat: &'a Grid<T>,
}

impl<T> Grid<T> {
    pub fn from_data(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(width * height, data.len());
        Self { width, height, data }
    }
    pub fn map_from_str(s: &str, f: impl Fn(char) -> T) -> Self {
        let width = s.lines().next().unwrap().len();
        let data: Vec<_> = s.chars().filter(|&c| !c.is_whitespace()).map(f).collect();
        Self::from_data(width, data.len() / width, data)
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn get(&self, p: Point) -> Option<&T> {
        if self.is_in_bounds(p) {
            Some(&self[p])
        } else {
            None
        }
    }
    pub fn is_in_bounds(&self, p: Point) -> bool {
        let bounds_x = self.width as isize;
        let bounds_y = self.height as isize;
        p.x >= 0 && p.x < bounds_x && p.y >= 0 && p.y < bounds_y
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
    pub fn enumerate(&self) -> GridEnumerate<'_,T> {
        GridEnumerate {
            iter: self.data.iter().enumerate(),
            mat: self,
        }
    }
    pub fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width, "x out of bounds");
        assert!(y < self.height, "y out of bounds");
        y * self.width + x
    }
    fn coords(&self, index: usize) -> Point {
        assert!(index < self.data.len(), "index out of bounds");
        let x = index % self.width;
        let y = index / self.width;
        Point::new(x as isize, y as isize)
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let data = vec![default; width * height];
        Self { width, height, data }
    }
    pub fn get_or(&self, p: Point, default: T) -> T {
        self.get(p).copied().unwrap_or(default)
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, elem: &T) -> Option<Point> {
        self.enumerate()
            .find(|&x| x.1 == elem)
            .map(|x| x.0)
    }
    pub fn find_all<'a>(&'a self, elem: &'a T) -> impl Iterator<Item = Point> + 'a {
        self.enumerate()
            .filter(move |&x| x.1 == elem)
            .map(|x| x.0)
    }
}

impl Grid<char> {
    pub fn from_str(s: &str) -> Self {
        Self::map_from_str(s, |c| c)
    }
}

impl<'a, T> Iterator for GridEnumerate<'a, T> {
    type Item = (Point, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, v)| (self.mat.coords(i), v))
    }
}

impl<T, I: PrimInt + Display> Index<(I, I)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (I, I)) -> &Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("x out of bounds: {}", x)),
            y.to_usize().unwrap_or_else(|| panic!("y out of bounds: {}", y)),
        );
        &self.data[i]
    }
}

impl<T, I: PrimInt + Display> IndexMut<(I, I)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (I, I)) -> &mut Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("x out of bounds: {}", x)),
            y.to_usize().unwrap_or_else(|| panic!("y out of bounds: {}", y)),
        );
        &mut self.data[i]
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Point) -> &Self::Output {
        &self[(pos.x, pos.y)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, pos: Point) -> &mut Self::Output {
        &mut self[(pos.x, pos.y)]
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
