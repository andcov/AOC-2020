// https://adventofcode.com/2020/day/1
use std::fs;

fn main() {
    println!("Day 01\nPart 1: {}\nPart 2: {}", solve_part_one().unwrap(), solve_part_two().unwrap());
}

fn solve_part_one() -> Option<i32> {
    let mut v = Vec::new();
    read_input(&mut v);

    for i in &v {
        let aux = 2020 - i;

        match v.binary_search(&aux) {
            Ok(_) => {
                return Some(i * aux);
            },
            Err(_) => {},
        };
    }

    None
}

fn solve_part_two() -> Option<i32> {
    let mut v = Vec::new();
    read_input(&mut v);

    for i in &v {
        for o in &v {
            let aux = 2020 - (i + o);

            match v.binary_search(&aux) {
                Ok(_) => {
                    return Some(i * o * aux);
                },
                Err(_) => {},
            };
        }
    }

    None
}

fn read_input(v: &mut Vec<i32>) {
   *v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split_whitespace().map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    v.sort();
}