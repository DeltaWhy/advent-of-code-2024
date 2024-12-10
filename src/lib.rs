use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[test]
fn test_vec2() {
    let v1 = Vec2 { x: 1, y: 2 };
    let v2 = Vec2 { x: 3, y: -4 };
    let v3 = Vec2 { x: 1, y: 2 };
    assert_eq!(v1 + v2, Vec2 { x: 4, y: -2 });
    assert_eq!(v1, v3);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub trait GridTrait
where
    Self: Sized,
{
    type Item;
    fn grid_get(&self, x: isize, y: isize) -> Option<Self::Item>;
    fn subgrid(&self, x: usize, y: usize, w: usize, h: usize) -> Option<Self>;
}

impl<'a> GridTrait for Vec<&'a str> {
    type Item = &'a str;
    fn grid_get(&self, x: isize, y: isize) -> Option<Self::Item> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        self.get(y)?.get(x..x + 1)
    }

    fn subgrid(&self, x: usize, y: usize, w: usize, h: usize) -> Option<Self> {
        let lines = self.get(y..y + h)?;
        let mut result = vec![];
        for line in lines {
            result.push(line.get(x..x + w)?);
        }
        Some(result)
    }
}

#[test]
fn test_grid_trait() {
    let grid = vec!["ABC", "DEF", "GHI"];
    assert_eq!(grid.grid_get(2, 0), Some("C"));
    assert_eq!(grid.grid_get(-1, 0), None);
    assert_eq!(grid.subgrid(0, 0, 2, 3), Some(vec!["AB", "DE", "GH"]));
    assert_eq!(grid.subgrid(1, 2, 2, 1), Some(vec!["HI"]));
    assert_eq!(grid.subgrid(0, 0, 4, 4), None);
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn as_vec2(self) -> Vec2<isize> {
        match self {
            Direction::Up => Vec2 { x: 0, y: -1 },
            Direction::Right => Vec2 { x: 1, y: 0 },
            Direction::Down => Vec2 { x: 0, y: 1 },
            Direction::Left => Vec2 { x: -1, y: 0 },
        }
    }
}

pub struct Grid {
    pub data: Vec<Vec<char>>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Grid {{")?;
        for line in &self.data {
            writeln!(f, "\t{}", line.iter().cloned().collect::<String>())?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.data {
            writeln!(f, "{}", line.iter().cloned().collect::<String>())?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn from(input: &str) -> Grid {
        Grid {
            data: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }
    pub fn grid_get(&self, x: isize, y: isize) -> Option<char> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        Some(*self.data.get(y)?.get(x)?)
    }
    pub fn subgrid(&self, x: usize, y: usize, w: usize, h: usize) -> Option<Self> {
        let lines = self.data.get(y..y + h)?;
        let mut result = vec![];
        for line in lines {
            result.push(line.get(x..x + w)?.to_owned());
        }
        Some(Grid { data: result })
    }
    pub fn iter_positions(&self) -> GridIterator {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
    pub fn rect(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: self.data[0].len(),
            h: self.data.len(),
        }
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl Iterator for GridIterator<'_> {
    type Item = ((usize, usize), char);
    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.data.len() {
            return None;
        }
        let c = self.grid.data[self.y][self.x];
        let res = Some(((self.x, self.y), c));
        self.x += 1;
        if self.x >= self.grid.data[self.y].len() {
            self.x = 0;
            self.y += 1;
        }
        res
    }
}

pub struct Rect {
    pub x: isize,
    pub y: isize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub fn contains(&self, point: Vec2<isize>) -> bool {
        (self.x..self.x + (self.w as isize)).contains(&point.x)
            && (self.y..self.y + (self.h as isize)).contains(&point.y)
    }
}
