use advent_of_code_2024::{Direction, Grid, Vec2};
use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, Read},
};

#[cfg(test)]
static TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

fn parse(input: &str) -> Grid {
    Grid::from(input)
}

fn region(grid: &Grid, visited: &mut HashSet<(usize, usize)>, x: usize, y: usize) -> usize {
    let mut queue: VecDeque<Vec2<isize>> = VecDeque::new();
    let mut area = 0;
    let mut perimeter = 0;
    let c = grid.data[y][x];
    queue.push_back(Vec2 {
        x: x as isize,
        y: y as isize,
    });
    while !queue.is_empty() {
        println!("{queue:?}");
        let pos = queue.pop_front().unwrap();
        area += 1;
        visited.insert((pos.x as usize, pos.y as usize));
        for dir in Direction::DIRECTIONS {
            let next_pos = pos + dir.as_vec2();
            if matches!(grid.grid_get(next_pos.x, next_pos.y), Some(x) if x == c) {
                if !visited.contains(&(next_pos.x as usize, next_pos.y as usize))
                    && !queue.contains(&next_pos)
                {
                    queue.push_back(next_pos);
                }
            } else {
                perimeter += 1;
            }
        }
    }
    area * perimeter
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut result = 0;
    grid.iter_positions().for_each(|((x, y), _)| {
        if visited.contains(&(x, y)) {
            return;
        }
        result += region(&grid, &mut visited, x, y);
    });
    result
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 1930);
}

#[allow(dead_code)]
fn part2(_input: &str) -> usize {
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
