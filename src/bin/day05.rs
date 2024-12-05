use std::{
    cmp::Ordering,
    io::{stdin, Read},
};

#[cfg(test)]
static TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[derive(Debug)]
struct Rule {
    earlier: i32,
    later: i32,
}

fn parse(input: &str) -> (Vec<Rule>, Vec<Vec<i32>>) {
    let rules = input
        .lines()
        .filter(|line| line.contains("|"))
        .map(|line| {
            let mut nums = line.splitn(2, "|").map(|word| word.parse::<i32>().unwrap());
            Rule {
                earlier: nums.next().unwrap(),
                later: nums.next().unwrap(),
            }
        })
        .collect();
    let updates: Vec<Vec<i32>> = input
        .lines()
        .filter(|line| line.contains(","))
        .map(|line| line.split(",").map(|word| word.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn satisfies(update: &[i32], rules: &[Rule]) -> bool {
    for rule in rules {
        if let (Some(i), Some(j)) = (
            update.iter().position(|&n| n == rule.earlier),
            update.iter().position(|&n| n == rule.later),
        ) {
            if i > j {
                return false;
            }
        }
    }
    true
}

fn part1(input: &str) -> i32 {
    let (rules, updates) = parse(input);
    updates
        .iter()
        .filter_map(|update| {
            if satisfies(update, &rules) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 143);
}

fn cmp(a: i32, b: i32, rules: &[Rule]) -> Option<Ordering> {
    for rule in rules {
        if rule.earlier == a && rule.later == b {
            return Some(Ordering::Less);
        } else if rule.earlier == b && rule.later == a {
            return Some(Ordering::Greater);
        }
    }
    None
}

fn reorder(update: &[i32], rules: &[Rule]) -> Vec<i32> {
    let mut update: Vec<i32> = update.to_owned();

    // bubble sort because std lacks a sort on PartialOrd, and n is small :)
    // pretty sure it needs to be a stable sort?
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..update.len() - 1 {
            if cmp(update[i], update[i + 1], rules) == Some(Ordering::Greater) {
                (update[i + 1], update[i]) = (update[i], update[i + 1]);
                swapped = true;
            }
        }
    }
    update
}

#[test]
fn test_reorder() {
    let (rules, _) = parse(TEST_INPUT);
    assert_eq!(
        reorder(&vec![75, 97, 47, 61, 53], &rules),
        vec![97, 75, 47, 61, 53]
    );
    assert_eq!(reorder(&vec![61, 13, 29], &rules), vec![61, 29, 13]);
    assert_eq!(
        reorder(&vec![97, 13, 75, 29, 47], &rules),
        vec![97, 75, 47, 29, 13]
    );
}

fn part2(input: &str) -> i32 {
    let (rules, updates) = parse(input);
    updates
        .iter()
        .filter_map(|update| {
            if satisfies(update, &rules) {
                None
            } else {
                Some(reorder(update, &rules))
            }
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 123);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
