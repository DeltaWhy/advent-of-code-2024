use regex::RegexBuilder;
use std::io::{stdin, Read};

#[cfg(test)]
static TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
#[cfg(test)]
static TEST_INPUT2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(i32, i32),
}

fn parse(input: &str) -> Vec<Instruction> {
    // let re = Regex::new(r"(?<op>do|don't|mul)\((?:(?<a>\d+),(?<b>\d+))?\)").unwrap();
    let re = RegexBuilder::new(r"
        (?<op>do|don't|mul)\(
            (?:(?<a>\d+),(?<b>\d+))?
        \)
    ")
        .ignore_whitespace(true)
        .build()
        .unwrap();
    let mut result = vec![];
    for cap in re.captures_iter(input) {
        let Some(op) = cap.name("op") else {
            continue;
        };
        match op.as_str() {
            "do" => result.push(Instruction::Do),
            "don't" => result.push(Instruction::Dont),
            "mul" => {
                let a = cap.name("a").unwrap().as_str().parse().unwrap();
                let b = cap.name("b").unwrap().as_str().parse().unwrap();
                result.push(Instruction::Mul(a, b));
            }
            _ => panic!("bad match"),
        }
    }
    result
}

fn part1(input: &str) -> i32 {
    let data = dbg!(parse(input));
    data.into_iter()
        .filter_map(|instr| match instr {
            Instruction::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 161);
}

fn part2(input: &str) -> i32 {
    let data = dbg!(parse(input));
    let mut result = 0;
    let mut enabled = true;
    for instr in data {
        match instr {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(a, b) => {
                if enabled {
                    result += a * b;
                }
            }
        }
    }
    result
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT2), 48);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
