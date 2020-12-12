// https://adventofcode.com/2020/day/13
use std::fs;

fn main() {
    println!("Day 13\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two())
}

fn solve_part_one() -> usize {}

fn solve_part_two() -> usize {}

fn read_input(v: &mut String) {
    *v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
}

