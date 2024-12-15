use std::io::{stdin, Read};

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn step(stones: &mut Vec<usize>) {
    let mut i = 0;
    while i < stones.len() {
        let stone = stones[i].to_string();
        if stone == "0" {
            stones[i] = 1;
            i += 1;
        } else if stone.len() % 2 == 0 {
            // let st1 = &stone[0..stone.len() / 2];
            // let st2 = &stone[stone.len() / 2..stone.len()];
            // stones[i] = st1.parse().unwrap();
            // stones.insert(i + 1, st2.parse().unwrap());
            // i += 2;
            let x = 10usize.pow(stone.len() as u32 / 2);
            let st1 = stones[i] / x;
            let st2 = stones[i] % x;
            stones[i] = st1;
            stones.insert(i + 1, st2);
            i += 2;
        } else {
            stones[i] *= 2024;
            i += 1;
        }
    }
}

#[test]
fn test_step() {
    let mut stones = parse("0 1 10 99 999");
    step(&mut stones);
    assert_eq!(stones, vec![1, 2024, 1, 0, 9, 9, 2021976]);
}

fn part1(input: &str) -> usize {
    let mut stones = parse(input);
    for _ in 0..25 {
        step(&mut stones);
    }
    stones.len()
}

#[test]
fn test_part1() {
    assert_eq!(part1("125 17"), 55312);
}

fn part2(input: &str) -> usize {
    let stones = parse(input);
    let mut result = 0;
    for stone in stones {
        let mut stones = vec![stone];
        for i in 0..75 {
            println!("{i}");
            step(&mut stones);
        }
        result += stones.len();
    }
    result
}

#[test]
#[ignore]
fn test_part2() {
    assert_eq!(part2(""), 0);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
