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

fn solve_machine2(machine: &ClawMachine) -> Option<usize> {
    // solve as a system of equations
    // ax*a + bx * b = px
    // ay*a + by * b = py
    //
    // ay*ax*a + ay*bx*b = ay*px
    // ax*ay*a + ax*by*b = ax*py
    // (ay*bx - ax*by)*b = ay*px - ax*py
    // b = (ay*px - ax*py) / (ay*bx - ax*by)
    // iff b is an integer
    // ax*a = px - bx*b
    // a = (px - bx*b) / ax
    let b_num = machine.a.y * machine.prize.x - machine.a.x * machine.prize.y;
    let b_denom = machine.a.y * machine.b.x - machine.a.x * machine.b.y;
    if b_denom == 0 {
        panic!("would divide by zero");
    }
    if b_num % b_denom != 0 {
        return None;
    }
    let b = b_num / b_denom;
    let a_num = machine.prize.x - machine.b.x * b;
    if a_num % machine.a.x != 0 {
        return None;
    }
    let a = a_num / machine.a.x;
    if a < 0 || b < 0 {
        panic!("got negative result");
    }
    Some(machine.cost.a * a as usize + machine.cost.b * b as usize)
}

#[test]
fn test_solve2() {
    assert_eq!(
        solve_machine2(&ClawMachine {
            a: Vec2 { x: 94, y: 34 },
            b: Vec2 { x: 22, y: 67 },
            prize: Vec2 {
                x: 1_000_000_000_8400,
                y: 1_000_000_000_5400
            },
            cost: Cost { a: 3, b: 1 }
        }),
        None
    );
    assert!(solve_machine2(&ClawMachine {
        a: Vec2 { x: 26, y: 66 },
        b: Vec2 { x: 67, y: 21 },
        prize: Vec2 {
            x: 1_000_000_001_2748,
            y: 1_000_000_001_2176
        },
        cost: Cost { a: 3, b: 1 }
    })
    .is_some());
    assert_eq!(
        solve_machine2(&ClawMachine {
            a: Vec2 { x: 17, y: 86 },
            b: Vec2 { x: 84, y: 37 },
            prize: Vec2 {
                x: 1_000_000_000_7870,
                y: 1_000_000_000_6450
            },
            cost: Cost { a: 3, b: 1 }
        }),
        None
    );
    assert!(solve_machine2(&ClawMachine {
        a: Vec2 { x: 69, y: 23 },
        b: Vec2 { x: 27, y: 71 },
        prize: Vec2 {
            x: 1_000_000_001_8641,
            y: 1_000_000_001_0279
        },
        cost: Cost { a: 3, b: 1 }
    })
    .is_some());
}

fn part2(input: &str) -> usize {
    let mut machines = parse(input);
    for machine in &mut machines {
        machine.prize.x += 10_000_000_000_000;
        machine.prize.y += 10_000_000_000_000;
    }
    machines.iter().filter_map(solve_machine2).sum()
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
    println!("Part 2: {}", part2(&input));
}
