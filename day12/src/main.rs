// https://adventofcode.com/2020/day/12
use std::fs;
use crate::Direction::*;

fn main() {
    println!("Day 12\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two())
}

//#[derive(PartialEq)]
enum Direction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn solve_part_one() -> usize {
    let mut v = Vec::new();
    read_input(&mut v);

    let mut n = 0;
    let mut e = 0;
    let mut dir = East(0);

    for i in &v {
        match *i {
            North(val) => n += val,
            South(val) => n -= val,
            East(val) => e += val,
            West(val) => e -= val,
            Left(val) => {
                let t = val / 90;
                for _ in 0..t {
                    dir = match dir {
                        North(_) => West(0),
                        West(_) => South(0),
                        South(_) => East(0),
                        East(_) => North(0),
                        _ => dir,
                    };
                }
            },
            Right(val) => {
                let t = val / 90;
                for _ in 0..t {
                    dir = match dir {
                        North(_) => East(0),
                        West(_) => North(0),
                        South(_) => West(0),
                        East(_) => South(0),
                        _ => dir,
                    };
                }
            },
            Forward(val) => {
                match dir {
                    North(_) => n += val,
                    South(_) => n -= val,
                    East(_) => e += val,
                    West(_) => e -= val,
                    _ => (),
                }
            },
        }
    }

    (n.abs() + e.abs()) as usize
}

fn solve_part_two() -> usize {
    let mut v = Vec::new();
    read_input(&mut v);

    let mut n = 0;
    let mut e = 0;
    let mut waypoint = (1, 10);

    for i in &v {
        match *i {
            North(val) => waypoint.0 += val,
            South(val) => waypoint.0 -= val,
            East(val) => waypoint.1 += val,
            West(val) => waypoint.1 -= val,
            Left(val) => {
                let t = val / 90;
                for _ in 0..t {
                    let aux = waypoint.0;
                    waypoint.0 = waypoint.1;
                    waypoint.1 = -aux;
                }
            },
            Right(val) => {
                let t = val / 90;
                for _ in 0..t {
                    let aux = waypoint.0;
                    waypoint.0 = -waypoint.1;
                    waypoint.1 = aux;
                }
            },
            Forward(val) => {
                n += waypoint.0 * val;
                e += waypoint.1 * val;
            },
        }
    }

    (n.abs() + e.abs()) as usize
}

fn read_input(v: &mut Vec<Direction>) {
    *v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines().map(|s| {
            let (d, i) = s.split_at(1);
            let dir;
            if d.eq("N") { dir = North(i.parse::<i32>().unwrap()) }
            else if d.eq("S") { dir = South(i.parse::<i32>().unwrap()) }
            else if d.eq("E") { dir = East(i.parse::<i32>().unwrap()) }
            else if d.eq("W") { dir = West(i.parse::<i32>().unwrap()) }
            else if d.eq("L") { dir = Left(i.parse::<i32>().unwrap()) }
            else if d.eq("R") { dir = Right(i.parse::<i32>().unwrap()) }
            else { dir = Forward(i.parse::<i32>().unwrap()) }
        dir
    }).collect();
}
