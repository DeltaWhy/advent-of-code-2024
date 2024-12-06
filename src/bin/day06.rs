use advent_of_code_2024::Vec2;
use core::fmt;
use std::{
    fmt::{Debug, Display, Formatter},
    io::{stdin, Read},
};

#[cfg(test)]
static TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn as_vec2(self) -> Vec2<isize> {
        match self {
            Direction::Up => Vec2 { x: 0, y: -1 },
            Direction::Right => Vec2 { x: 1, y: 0 },
            Direction::Down => Vec2 { x: 0, y: 1 },
            Direction::Left => Vec2 { x: -1, y: 0 },
        }
    }
}

struct Grid {
    data: Vec<Vec<char>>,
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
    fn grid_get(&self, x: isize, y: isize) -> Option<char> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        Some(*self.data.get(y)?.get(x)?)
    }
    #[allow(dead_code)]
    fn subgrid(&self, x: usize, y: usize, w: usize, h: usize) -> Option<Self> {
        let lines = self.data.get(y..y + h)?;
        let mut result = vec![];
        for line in lines {
            result.push(line.get(x..x + w)?.to_owned());
        }
        Some(Grid { data: result })
    }
}

fn parse(input: &str) -> Grid {
    Grid {
        data: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

fn part1(input: &str) -> i32 {
    let mut grid = parse(input);
    // println!("{grid}");
    let mut guard_pos: Option<Vec2<isize>> = None;
    'outer: for y in 0..grid.data.len() {
        let line = &grid.data[y];
        #[allow(clippy::needless_range_loop)]
        for x in 0..line.len() {
            if line[x] == '^' {
                guard_pos = Some(Vec2 {
                    x: x as isize,
                    y: y as isize,
                });
                break 'outer;
            }
        }
    }
    let mut guard_pos: Vec2<isize> = guard_pos.expect("guard position not found");
    grid.data[guard_pos.y as usize][guard_pos.x as usize] = 'X';
    let mut guard_direction: Direction = Direction::Up;
    // println!("{guard_pos:?} {guard_direction:?}");
    let mut result = 1;
    loop {
        let next_pos = guard_pos + guard_direction.as_vec2();
        match grid.grid_get(next_pos.x, next_pos.y) {
            Some('#') => {
                guard_direction = guard_direction.turn_right();
            }
            Some(c) => {
                guard_pos = next_pos;
                grid.data[guard_pos.y as usize][guard_pos.x as usize] = 'X';
                // println!("{grid}");
                if c != 'X' {
                    result += 1;
                }
            }
            None => break,
        }
    }
    result
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 41);
}

#[allow(dead_code)]
fn part2(_input: &str) -> i32 {
    todo!();
}

#[test]
#[ignore]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 0);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}
