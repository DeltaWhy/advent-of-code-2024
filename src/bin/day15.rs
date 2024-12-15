use advent_of_code_2024::{Direction, Grid, Vec2};
use std::io::{stdin, Read};

#[cfg(test)]
static TEST_INPUT_SMALL: &str = include_str!("test15_small.txt");
#[cfg(test)]
static TEST_INPUT_LARGE: &str = include_str!("test15_large.txt");

fn parse(input: &str) -> (Grid, Vec<Direction>) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let moves = moves
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            c => {
                if c.is_whitespace() {
                    None
                } else {
                    panic!("expected valid move")
                }
            }
        })
        .collect();
    (Grid::from(grid), moves)
}

fn do_move(grid: &mut Grid, pos: Vec2<isize>, direction: Direction) -> bool {
    let next_pos = pos + direction.as_vec2();
    match grid.grid_get(next_pos.x, next_pos.y) {
        None => false,
        Some('#') => false,
        Some('.') => {
            grid.data[next_pos.y as usize][next_pos.x as usize] =
                grid.data[pos.y as usize][pos.x as usize];
            grid.data[pos.y as usize][pos.x as usize] = '.';
            true
        }
        Some('O') => {
            if do_move(grid, next_pos, direction) {
                grid.data[next_pos.y as usize][next_pos.x as usize] =
                    grid.data[pos.y as usize][pos.x as usize];
                grid.data[pos.y as usize][pos.x as usize] = '.';
                true
            } else {
                false
            }
        }
        _ => panic!("unknown character in grid"),
    }
}

fn part1(input: &str) -> usize {
    let (mut grid, moves) = parse(input);
    // println!("{grid}");
    for mov in moves {
        let (robot_pos, _) = grid.iter_positions().find(|(_, c)| *c == '@').unwrap();
        let robot_pos = Vec2 {
            x: robot_pos.0 as isize,
            y: robot_pos.1 as isize,
        };
        do_move(&mut grid, robot_pos, mov);
        // println!("{grid}");
    }
    grid.iter_positions()
        .map(|((x, y), c)| if c == 'O' { x + 100 * y } else { 0 })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT_SMALL), 2028);
    assert_eq!(part1(TEST_INPUT_LARGE), 10092);
}

fn widen(grid: Grid) -> Grid {
    let mut result: Grid = Grid::of('.', grid.rect().w * 2, grid.rect().h);
    for ((x, y), c) in grid.iter_positions() {
        match c {
            '#' => {
                result.data[y][x * 2] = '#';
                result.data[y][x * 2 + 1] = '#';
            }
            'O' => {
                result.data[y][x * 2] = '[';
                result.data[y][x * 2 + 1] = ']';
            }
            '.' => {
                result.data[y][x * 2] = '.';
                result.data[y][x * 2 + 1] = '.';
            }
            '@' => {
                result.data[y][x * 2] = '@';
                result.data[y][x * 2 + 1] = '.';
            }
            _ => panic!("unknown char in grid"),
        }
    }
    result
}

fn can_move2(grid: &Grid, pos: Vec2<isize>, direction: Direction) -> bool {
    let next_pos = pos + direction.as_vec2();
    match grid.grid_get(pos.x, pos.y) {
        None => false,
        Some('#') => false,
        Some('.') => true,
        Some('[') => match direction {
            Direction::Left | Direction::Right => can_move2(grid, next_pos, direction),
            Direction::Up | Direction::Down => {
                can_move2(grid, next_pos, direction)
                    && can_move2(grid, next_pos + Vec2 { x: 1, y: 0 }, direction)
            }
        },
        Some(']') => match direction {
            Direction::Left | Direction::Right => can_move2(grid, next_pos, direction),
            Direction::Up | Direction::Down => {
                can_move2(grid, next_pos, direction)
                    && can_move2(grid, next_pos + Vec2 { x: -1, y: 0 }, direction)
            }
        },
        Some('@') => can_move2(grid, next_pos, direction),
        _ => panic!("unknown character in grid"),
    }
}

fn do_move2(grid: &mut Grid, pos: Vec2<isize>, direction: Direction) {
    let next_pos = pos + direction.as_vec2();
    let c = grid.grid_get(pos.x, pos.y).unwrap();
    println!("{c} {pos:?} {next_pos:?} {direction:?}");
    match direction {
        Direction::Left | Direction::Right => match grid.grid_get(next_pos.x, next_pos.y) {
            Some('.') => {
                grid.data[next_pos.y as usize][next_pos.x as usize] = c;
                grid.data[pos.y as usize][pos.x as usize] = '.';
            }
            Some('[') | Some(']') => {
                do_move2(grid, next_pos, direction);
                grid.data[next_pos.y as usize][next_pos.x as usize] = c;
                grid.data[pos.y as usize][pos.x as usize] = '.';
            }
            _ => panic!("do_move2 was called but can't move"),
        },
        Direction::Up | Direction::Down => {
            match grid.grid_get(next_pos.x, next_pos.y) {
                Some('.') => {
                    grid.data[next_pos.y as usize][next_pos.x as usize] = c;
                    grid.data[pos.y as usize][pos.x as usize] = '.';
                }
                Some('[') | Some(']') => {
                    do_move2(grid, next_pos, direction);
                    println!("{grid:?}");
                    grid.data[next_pos.y as usize][next_pos.x as usize] = c;
                    grid.data[pos.y as usize][pos.x as usize] = '.';
                }
                _ => panic!("do_move2 was called but can't move"),
            }
            if c == '[' && grid.grid_get(next_pos.x, next_pos.y) != Some('[') {
                do_move2(grid, pos + Vec2 { x: 1, y: 0 }, direction);
            } else if c == ']' && grid.grid_get(next_pos.x, next_pos.y) != Some(']') {
                do_move2(grid, pos + Vec2 { x: -1, y: 0 }, direction);
            } else if c == '@' && grid.grid_get(next_pos.x, next_pos.y) == Some('[') {
                do_move2(grid, pos + Vec2 { x: 1, y: 0 }, direction);
            } else if c == '@' && grid.grid_get(next_pos.x, next_pos.y) == Some(']') {
                do_move2(grid, pos + Vec2 { x: -1, y: 0 }, direction);
            }
        }
    }
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    let (grid, moves) = parse(input);
    let mut grid = widen(grid);
    println!("{grid}");
    for mov in moves {
        let (robot_pos, _) = grid.iter_positions().find(|(_, c)| *c == '@').unwrap();
        let robot_pos = Vec2 {
            x: robot_pos.0 as isize,
            y: robot_pos.1 as isize,
        };
        println!("{mov:?}");
        if can_move2(&grid, robot_pos, mov) {
            do_move2(&mut grid, robot_pos, mov);
        }
        println!("{grid}");
    }
    todo!();
}

#[test]
#[ignore]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT_LARGE), 9021);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}
