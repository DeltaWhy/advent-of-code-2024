use advent_of_code_2024::{Direction, Grid, Vec2};
use std::io::{stdin, Read};

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
