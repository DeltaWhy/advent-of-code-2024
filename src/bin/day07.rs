use std::io::{stdin, Read};

#[cfg(test)]
static TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let mut line = line.splitn(2, ": ");
            let expected: i64 = line.next().unwrap().parse().unwrap();
            let nums: Vec<i64> = line
                .next()
                .unwrap()
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect();
            (expected, nums)
        })
        .collect()
}

fn is_satisfiable(expected: i64, nums: &[i64]) -> bool {
    // iterate all possible op values - 0 bit is +, 1 bit is *
    let num_ops: usize = nums.len() - 1;
    let n = 2i32.pow(num_ops as u32);
    for i in 0..n {
        let mut acc = nums[0];
        let mut i = i;
        for j in 0..num_ops {
            if i & 1 == 1 {
                acc *= nums[j + 1];
            } else {
                acc += nums[j + 1];
            }
            i >>= 1;
        }
        if acc == expected {
            return true;
        }
    }
    false
}

fn part1(input: &str) -> i64 {
    let equations = parse(input);
    equations
        .iter()
        .filter(|(expected, nums)| is_satisfiable(*expected, nums))
        .map(|(expected, _)| expected)
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 3749);
}

fn is_satisfiable2(expected: i64, nums: &[i64]) -> bool {
    // iterate all possible op values - 0 is +, 1 is *, 2 is concat
    // is there a faster way than brute force?
    let num_ops: usize = nums.len() - 1;
    let n = 3i32.pow(num_ops as u32);
    for i in 0..n {
        let mut acc = nums[0];
        let mut i = i;
        for j in 0..num_ops {
            match i % 3 {
                0 => {
                    acc += nums[j + 1];
                }
                1 => {
                    acc *= nums[j + 1];
                }
                2 => {
                    // concat
                    acc = format!("{acc}{}", nums[j + 1]).parse().unwrap();
                }
                _ => unreachable!(),
            }
            i /= 3;
        }
        if acc == expected {
            return true;
        }
    }
    false
}

#[test]
fn test_is_satisfiable2() {
    assert!(is_satisfiable2(156, &vec![15, 6]));
    assert!(is_satisfiable2(7290, &vec![6, 8, 6, 15]));
    assert!(is_satisfiable2(192, &vec![17, 8, 14]));
}

fn part2(input: &str) -> i64 {
    let equations = parse(input);
    equations
        .iter()
        .filter(|(expected, nums)| is_satisfiable2(*expected, nums))
        .map(|(expected, _)| expected)
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 11387);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
