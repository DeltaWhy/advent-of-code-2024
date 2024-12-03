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
