use advent_of_code_2024::Vec2;
use std::io::{stdin, Read};

#[cfg(test)]
static TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

#[derive(Debug)]
struct Cost {
    a: usize,
    b: usize,
}

#[derive(Debug)]
struct ClawMachine {
    a: Vec2<isize>,
    b: Vec2<isize>,
    prize: Vec2<isize>,
    cost: Cost,
}

fn parse(input: &str) -> Vec<ClawMachine> {
    fn next_number(input: &mut &str) -> Option<usize> {
        while input.chars().next().is_some_and(|c| !c.is_ascii_digit()) {
            *input = &input[1..];
        }
        let mut end = 0;
        while end < input.len() {
            if input[end..end + 1]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_digit())
            {
                end += 1;
            } else {
                break;
            }
        }
        if end > 0 {
            let token = &input[0..end];
            *input = &input[end..];
            token.parse().ok()
        } else {
            None
        }
    }
    let mut ptr: &str = input;
    let mut result = Vec::new();
    while let Some(ax) = next_number(&mut ptr) {
        result.push(ClawMachine {
            a: Vec2 {
                x: ax as isize,
                y: next_number(&mut ptr).unwrap() as isize,
            },
            b: Vec2 {
                x: next_number(&mut ptr).unwrap() as isize,
                y: next_number(&mut ptr).unwrap() as isize,
            },
            prize: Vec2 {
                x: next_number(&mut ptr).unwrap() as isize,
                y: next_number(&mut ptr).unwrap() as isize,
            },
            cost: Cost { a: 3, b: 1 },
        })
    }
    result
}

fn solve_machine(machine: &ClawMachine) -> Option<usize> {
    let mut sol = None;
    for a in 0..=100 {
        for b in 0..=100 {
            if a * machine.a.x + b * machine.b.x == machine.prize.x
                && a * machine.a.y + b * machine.b.y == machine.prize.y
            {
                let cost = (a as usize) * machine.cost.a + (b as usize) * machine.cost.b;
                match sol {
                    None => {
                        sol = Some(cost);
                    }
                    Some(prev_cost) => {
                        if cost < prev_cost {
                            sol = Some(cost);
                        }
                    }
                }
            }
        }
    }
    sol
}

fn part1(input: &str) -> usize {
    let machines = parse(input);
    machines.iter().filter_map(solve_machine).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 480);
}

#[allow(dead_code)]
fn part2(_input: &str) -> i32 {
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
