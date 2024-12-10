use advent_of_code_2024::{Direction, Grid, Vec2};
use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, Read},
};

#[cfg(test)]
static TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

fn parse(input: &str) -> Grid {
    Grid::from(input)
}

fn trailhead_score(grid: &Grid, trailhead: &Vec2<isize>) -> usize {
    let mut q: VecDeque<Vec2<isize>> = VecDeque::from([*trailhead]);
    let mut reachable: HashSet<Vec2<isize>> = HashSet::new();
    while !q.is_empty() {
        let pos = q.pop_front().unwrap();
        let c = grid.grid_get(pos.x, pos.y).unwrap();
        if c == '9' {
            reachable.insert(pos);
            continue;
        }
        let next_c = c
            .to_digit(10)
            .map(|x| char::from_digit(x + 1, 10).unwrap())
            .unwrap();
        for dir in Direction::DIRECTIONS {
            let next_pos = pos + dir.as_vec2();
            if grid.grid_get(next_pos.x, next_pos.y) == Some(next_c) {
                q.push_back(next_pos);
            }
        }
    }
    reachable.len()
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let trailheads: Vec<Vec2<isize>> = grid
        .iter_positions()
        .filter_map(|((x, y), c)| match c {
            '0' => Some(Vec2 {
                x: x as isize,
                y: y as isize,
            }),
            _ => None,
        })
        .collect();
    trailheads.iter().map(|t| trailhead_score(&grid, t)).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 36);
}

fn trailhead_rating(grid: &Grid, trailhead: &Vec2<isize>) -> usize {
    let mut q: VecDeque<Vec2<isize>> = VecDeque::from([*trailhead]);
    let mut rating = 0;
    while !q.is_empty() {
        let pos = q.pop_front().unwrap();
        let c = grid.grid_get(pos.x, pos.y).unwrap();
        if c == '9' {
            rating += 1;
            continue;
        }
        let next_c = c
            .to_digit(10)
            .map(|x| char::from_digit(x + 1, 10).unwrap())
            .unwrap();
        for dir in Direction::DIRECTIONS {
            let next_pos = pos + dir.as_vec2();
            if grid.grid_get(next_pos.x, next_pos.y) == Some(next_c) {
                q.push_back(next_pos);
            }
        }
    }
    rating
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    let trailheads: Vec<Vec2<isize>> = grid
        .iter_positions()
        .filter_map(|((x, y), c)| match c {
            '0' => Some(Vec2 {
                x: x as isize,
                y: y as isize,
            }),
            _ => None,
        })
        .collect();
    trailheads.iter().map(|t| trailhead_rating(&grid, t)).sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 81);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
