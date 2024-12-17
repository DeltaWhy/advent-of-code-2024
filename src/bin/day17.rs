use std::io::{stdin, Read};

#[cfg(test)]
static TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

#[cfg(test)]
static TEST_INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

#[derive(Debug)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
}

fn parse(input: &str) -> (Registers, Vec<usize>) {
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
    let registers = Registers {
        a: next_number(&mut ptr).unwrap(),
        b: next_number(&mut ptr).unwrap(),
        c: next_number(&mut ptr).unwrap(),
        pc: 0,
    };
    let (_, program) = input.split_once("Program: ").unwrap();
    let program = program
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    (registers, program)
}

fn combo_value(registers: &Registers, operand: usize) -> usize {
    match operand {
        0..=3 => operand,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!("illegal combo operand"),
    }
}

fn part1(input: &str) -> String {
    let (mut registers, program) = parse(input);
    println!("{registers:?} {program:?}");
    let mut output: Vec<usize> = vec![];
    while registers.pc < program.len() {
        step(&program, &mut registers, &mut output);
    }
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn step(program: &[usize], registers: &mut Registers, output: &mut Vec<usize>) {
    let op = program[registers.pc];
    match op {
        0 => {
            // adv
            let combo = program[registers.pc + 1];
            println!("adv {combo}");
            registers.a >>= combo_value(registers, combo);
            registers.pc += 2;
        }
        1 => {
            // bxl
            let literal = program[registers.pc + 1];
            println!("bxl {literal}");
            registers.b ^= literal;
            registers.pc += 2;
        }
        2 => {
            // bst
            let combo = program[registers.pc + 1];
            println!("bst {combo}");
            registers.b = combo_value(&*registers, combo) % 8;
            registers.pc += 2;
        }
        3 => {
            // jnz
            if registers.a == 0 {
                println!("jnz (a==0)");
                registers.pc += 2;
            } else {
                let literal = program[registers.pc + 1];
                println!("jnz {literal} (a!=0)");
                registers.pc = literal;
            }
        }
        4 => {
            // bxc
            println!("bxc");
            registers.b ^= registers.c;
            registers.pc += 2;
        }
        5 => {
            // out
            let combo = program[registers.pc + 1];
            println!("out {combo}");
            output.push(combo_value(&*registers, combo) % 8);
            registers.pc += 2;
        }
        6 => {
            // bdv
            let combo = program[registers.pc + 1];
            println!("bdv {combo}");
            registers.b = registers.a >> combo_value(&*registers, combo);
            registers.pc += 2;
        }
        7 => {
            // cdv
            let combo = program[registers.pc + 1];
            println!("cdv {combo}");
            registers.c = registers.a >> combo_value(&*registers, combo);
            registers.pc += 2;
        }
        _ => panic!("illegal opcode"),
    }
    println!("{registers:?} {output:?}");
}

#[test]
fn test_step() {
    let mut registers = Registers {
        a: 0,
        b: 0,
        c: 9,
        pc: 0,
    };
    let mut output: Vec<usize> = vec![];
    step(&vec![2, 6], &mut registers, &mut output);
    assert_eq!(registers.b, 1);

    let program = vec![5, 0, 5, 1, 5, 4];
    registers = Registers {
        a: 10,
        b: 0,
        c: 0,
        pc: 0,
    };
    while registers.pc < program.len() {
        step(&program, &mut registers, &mut output);
    }
    assert_eq!(output, vec![0, 1, 2]);

    let program = vec![0, 1, 5, 4, 3, 0];
    output = vec![];
    registers = Registers {
        a: 2024,
        b: 0,
        c: 0,
        pc: 0,
    };
    while registers.pc < program.len() {
        step(&program, &mut registers, &mut output);
    }
    assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    assert_eq!(registers.a, 0);

    let program = vec![1, 7];
    output = vec![];
    registers = Registers {
        a: 0,
        b: 29,
        c: 0,
        pc: 0,
    };
    while registers.pc < program.len() {
        step(&program, &mut registers, &mut output);
    }
    assert_eq!(registers.b, 26);

    let program = vec![4, 0];
    output = vec![];
    registers = Registers {
        a: 0,
        b: 2024,
        c: 43690,
        pc: 0,
    };
    while registers.pc < program.len() {
        step(&program, &mut registers, &mut output);
    }
    assert_eq!(registers.b, 44354);
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), "4,6,3,5,6,3,5,2,1,0");
}

#[allow(dead_code)]
fn part2(_input: &str) -> usize {
    todo!();
}

#[test]
#[ignore]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT2), 0);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}
