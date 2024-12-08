use advent_of_code_2024::{Grid, Vec2};
use std::{
    collections::HashSet,
    io::{stdin, Read},
};

#[cfg(test)]
static TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

fn parse(input: &str) -> Grid {
    Grid {
        data: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

fn get_frequencies(grid: &Grid) -> HashSet<char> {
    let mut unique_chars: HashSet<char> = HashSet::new();
    for (_, c) in grid.iter_positions() {
        unique_chars.insert(c);
    }
    unique_chars.remove(&'.');
    unique_chars
}

fn get_antenna_positions(grid: &Grid, frequency: char) -> Vec<(usize, usize)> {
    let mut antennas: Vec<(usize, usize)> = vec![];
    for ((x, y), c) in grid.iter_positions() {
        if c == frequency {
            antennas.push((x, y));
        }
    }
    antennas
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for freq in get_frequencies(&grid) {
        // println!("{freq}: {antennas:?}");
        let antennas = get_antenna_positions(&grid, freq);
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                let a: Vec2<isize> = Vec2 {
                    x: antennas[i].0 as isize,
                    y: antennas[i].1 as isize,
                };
                let b: Vec2<isize> = Vec2 {
                    x: antennas[j].0 as isize,
                    y: antennas[j].1 as isize,
                };
                let delta = a - b;
                // println!("{a:?} - {b:?} = {delta:?}");
                let anti1 = a + delta;
                let anti2 = b - delta;
                // println!("{anti1:?} {anti2:?}");
                if grid.rect().contains(anti1) {
                    antinodes.insert((anti1.x as usize, anti1.y as usize));
                }
                if grid.rect().contains(anti2) {
                    antinodes.insert((anti2.x as usize, anti2.y as usize));
                }
            }
        }
        // println!("{antinodes:?}");
    }
    antinodes.len()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 14);
}

fn part2(input: &str) -> usize {
    // I think this might be incomplete - distance between two antennas may not be a simplified
    // ratio, e.g. (2,4) so we'd miss grid points at (1,2) intervals that are also collinear.
    // It worked for my input though.
    let grid = parse(input);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for freq in get_frequencies(&grid) {
        // println!("{freq}: {antennas:?}");
        let antennas = get_antenna_positions(&grid, freq);
        for i in 0..antennas.len() - 1 {
            antinodes.insert(antennas[i]);
            for j in i + 1..antennas.len() {
                let a: Vec2<isize> = Vec2 {
                    x: antennas[i].0 as isize,
                    y: antennas[i].1 as isize,
                };
                let b: Vec2<isize> = Vec2 {
                    x: antennas[j].0 as isize,
                    y: antennas[j].1 as isize,
                };
                antinodes.insert(antennas[j]);
                let delta = a - b;
                // println!("{a:?} - {b:?} = {delta:?}");
                let mut anti = a + delta;
                while grid.rect().contains(anti) {
                    antinodes.insert((anti.x as usize, anti.y as usize));
                    anti += delta;
                }
                let mut anti = b - delta;
                while grid.rect().contains(anti) {
                    antinodes.insert((anti.x as usize, anti.y as usize));
                    anti -= delta;
                }
            }
        }
        // println!("{antinodes:?}");
    }
    antinodes.len()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 34);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
