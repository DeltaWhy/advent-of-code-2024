use advent_of_code_2024::GridTrait;
use std::io::{stdin, Read};

#[cfg(test)]
static TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(input: &str) -> i32 {
    let input = parse(input);
    let mut result = 0;

    // horizontal search
    for line in &input {
        for i in 0..line.len() {
            if line[i..].starts_with("XMAS") || line[i..].starts_with("SAMX") {
                result += 1;
            }
        }
    }

    // vertical search
    for i in 0..(input.len() - 3) {
        for j in 0..input[i].len() {
            if input[i][j..].starts_with("X")
                && input[i + 1][j..].starts_with("M")
                && input[i + 2][j..].starts_with("A")
                && input[i + 3][j..].starts_with("S")
                || input[i][j..].starts_with("S")
                    && input[i + 1][j..].starts_with("A")
                    && input[i + 2][j..].starts_with("M")
                    && input[i + 3][j..].starts_with("X")
            {
                result += 1;
            }
        }
    }

    // diagonal search
    for i in 0..(input.len() - 3) {
        for j in 0..input[i].len() {
            let i: isize = i as isize;
            let j: isize = j as isize;
            match (
                input.grid_get(j, i),
                input.grid_get(j + 1, i + 1),
                input.grid_get(j + 2, i + 2),
                input.grid_get(j + 3, i + 3),
            ) {
                (Some("X"), Some("M"), Some("A"), Some("S"))
                | (Some("S"), Some("A"), Some("M"), Some("X")) => {
                    result += 1;
                }
                _ => {}
            }
            match (
                input.grid_get(j, i),
                input.grid_get(j - 1, i + 1),
                input.grid_get(j - 2, i + 2),
                input.grid_get(j - 3, i + 3),
            ) {
                (Some("X"), Some("M"), Some("A"), Some("S"))
                | (Some("S"), Some("A"), Some("M"), Some("X")) => {
                    result += 1;
                }
                _ => {}
            }
        }
    }

    result
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 18);
}

fn part2(input: &str) -> i32 {
    let input = parse(input);
    let mut result = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let x: isize = x as isize;
            let y: isize = y as isize;
            if input.grid_get(x + 1, y + 1) != Some("A") {
                continue;
            }
            let a = input.grid_get(x, y);
            let b = input.grid_get(x + 2, y + 2);
            let c = input.grid_get(x + 2, y);
            let d = input.grid_get(x, y + 2);
            if matches!((a, b), (Some("M"), Some("S")) | (Some("S"), Some("M")))
                && matches!((c, d), (Some("M"), Some("S")) | (Some("S"), Some("M")))
            {
                result += 1;
            }
        }
    }
    result
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 9);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
