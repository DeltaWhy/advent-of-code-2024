use std::{
    collections::HashMap,
    io::{stdin, Read},
    iter::zip,
};

#[cfg(test)]
static TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let i1: i32 = words[0].parse().expect("not an integer");
        let i2: i32 = words[1].parse().expect("not an integer");
        list1.push(i1);
        list2.push(i2);
    }

    (list1, list2)
}

fn part1(input: &str) -> i32 {
    let (mut list1, mut list2) = parse(input);
    list1.sort();
    list2.sort();

    zip(list1, list2).map(|(a, b)| (a - b).abs()).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 11);
}

fn part2(input: &str) -> i32 {
    let (list1, list2) = parse(input);
    let mut occurrences = HashMap::<i32, i32>::new();

    for x in list2 {
        occurrences.insert(x, occurrences.get(&x).unwrap_or(&0) + 1);
    }

    list1
        .iter()
        .map(|x| x * occurrences.get(x).unwrap_or(&0))
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 31);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
