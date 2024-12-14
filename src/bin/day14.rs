use std::{io::{stdin, Read}, thread::sleep, time::Duration};
use advent_of_code_2024::{Vec2, Grid};

#[cfg(test)]
static TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

#[derive(Debug)]
struct Robot {
    p: Vec2<isize>,
    v: Vec2<isize>
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(|line| {
        let (p_str, v_str) = line.trim().split_once(" ").unwrap();
        let (_, p_str) = p_str.split_once("=").unwrap();
        let (px, py) = p_str.split_once(",").unwrap();
        let (_, v_str) = v_str.split_once("=").unwrap();
        let (vx, vy) = v_str.split_once(",").unwrap();
        Robot {
            p: Vec2 { x: px.parse().unwrap(), y: py.parse().unwrap() },
            v: Vec2 { x: vx.parse().unwrap(), y: vy.parse().unwrap() }
        }
    }).collect()
}

fn part1(input: &str, w: isize, h: isize) -> isize {
    let mut robots = parse(input);
    for robot in &mut robots {
        let mut x = (robot.p.x + robot.v.x * 100) % w;
        if x < 0 {
            x += w;
        }
        let mut y = (robot.p.y + robot.v.y * 100) % h;
        if y < 0 {
            y += h;
        }
        robot.p.x = x;
        robot.p.y = y;
    }
    let q1: isize = robots.iter().map(|robot| {
        if robot.p.x < w/2 && robot.p.y < h/2 {
            1
        } else {
            0
        }
    }).sum();
    let q2: isize = robots.iter().map(|robot| {
        if robot.p.x > w/2 && robot.p.y < h/2 {
            1
        } else {
            0
        }
    }).sum();
    let q3: isize = robots.iter().map(|robot| {
        if robot.p.x < w/2 && robot.p.y > h/2 {
            1
        } else {
            0
        }
    }).sum();
    let q4: isize = robots.iter().map(|robot| {
        if robot.p.x > w/2 && robot.p.y > h/2 {
            1
        } else {
            0
        }
    }).sum();
    q1 * q2 * q3 * q4
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT, 11, 7), 12);
}

fn part2(input: &str, w: isize, h: isize) {
    let mut robots = parse(input);
    // let mut t = 0;
    let show_grid = |robots: &Vec<Robot>| {
        let mut grid = Grid::of('.', w as usize, h as usize);
        for robot in robots {
            grid.data[robot.p.y as usize][robot.p.x as usize] = 'X';
        }
        println!("{grid}");
    };
    for t in 0..10000 {
        show_grid(&robots);
        println!("{t}");
        // t += 1;
        for robot in &mut robots {
            robot.p.x = (robot.p.x + robot.v.x) % w;
            robot.p.y = (robot.p.y + robot.v.y) % h;
            if robot.p.x < 0 {
                robot.p.x += w;
            }
            if robot.p.y < 0 {
                robot.p.y += h;
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input, 101, 103));
    part2(&input, 101, 103);
}
