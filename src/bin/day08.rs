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

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut unique_chars: HashSet<char> = HashSet::new();
    for (_, c) in grid.iter_positions() {
        unique_chars.insert(c);
    }
    unique_chars.remove(&'.');
    // println!("{unique_chars:?}");
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for freq in unique_chars {
        let mut antennas: Vec<(usize, usize)> = vec![];
        for ((x, y), c) in grid.iter_positions() {
            if c == freq {
                antennas.push((x, y));
            }
        }
        // println!("{freq}: {antennas:?}");
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

fn part2(_input: &str) -> usize {
    0
}

#[test]
#[ignore]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 34);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
