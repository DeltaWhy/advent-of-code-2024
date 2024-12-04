use std::io::{stdin, Read};
use std::iter::zip;

#[cfg(test)]
static TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<i32>().expect("should be integer"))
                .collect()
        })
        .collect()
}

fn is_safe(report: &&Vec<i32>) -> bool {
    if report.len() < 2 {
        return false;
    }

    let pairs = zip(&report[0..report.len() - 1], &report[1..]);
    let diffs = pairs.map(|(a, b)| b - a);

    diffs.clone().all(|item| (1..=3).contains(&item))
        || diffs.clone().all(|item| (-3..=-1).contains(&item))
}

fn part1(input: &str) -> i32 {
    let reports = parse(input);
    reports.iter().filter(is_safe).count().try_into().unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 2);
}

fn is_safe2(report: &&Vec<i32>) -> bool {
    if report.len() < 2 {
        return false;
    }

    let pairs = zip(&report[0..report.len() - 1], &report[1..]);
    let diffs: Vec<i32> = pairs.map(|(a, b)| b - a).collect();

    // try increasing
    match diffs.iter().position(|diff| !(1..=3).contains(diff)) {
        None => return true,
        Some(i) => {
            let mut try1: Vec<i32> = report.to_vec();
            try1.remove(i);
            if is_safe(&&try1) {
                return true;
            }
            let mut try2: Vec<i32> = report.to_vec();
            try2.remove(i + 1);
            if is_safe(&&try2) {
                return true;
            }
        }
    }
    // try decreasing
    match diffs.iter().position(|diff| !(-3..=-1).contains(diff)) {
        None => return true,
        Some(i) => {
            let mut try1: Vec<i32> = report.to_vec();
            try1.remove(i);
            if is_safe(&&try1) {
                return true;
            }
            let mut try2: Vec<i32> = report.to_vec();
            try2.remove(i + 1);
            if is_safe(&&try2) {
                return true;
            }
        }
    }

    false
}

#[test]
fn test_safe2() {
    assert!(is_safe2(&&vec![1, 5, 3, 4, 5]));
    assert!(is_safe2(&&vec![1, 4, 3, 4, 5]));
}

fn part2(input: &str) -> i32 {
    let reports = parse(input);
    reports.iter().filter(is_safe2).count().try_into().unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 4);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
