use std::ops::Add;

#[test]
fn test_vec2() {
    let v1 = Vec2 { x: 1, y: 2 };
    let v2 = Vec2 { x: 3, y: -4 };
    let v3 = Vec2 { x: 1, y: 2 };
    assert_eq!(v1 + v2, Vec2 { x: 4, y: -2 });
    assert_eq!(v1, v3);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

pub trait Grid
where
    Self: Sized,
{
    type Item;
    fn grid_get(&self, x: isize, y: isize) -> Option<Self::Item>;
    fn subgrid(&self, x: usize, y: usize, w: usize, h: usize) -> Option<Self>;
}

impl<'a> Grid for Vec<&'a str> {
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
fn test_grid() {
    let grid = vec!["ABC", "DEF", "GHI"];
    assert_eq!(grid.grid_get(2, 0), Some("C"));
    assert_eq!(grid.grid_get(-1, 0), None);
    assert_eq!(grid.subgrid(0, 0, 2, 3), Some(vec!["AB", "DE", "GH"]));
    assert_eq!(grid.subgrid(1, 2, 2, 1), Some(vec!["HI"]));
    assert_eq!(grid.subgrid(0, 0, 4, 4), None);
}
