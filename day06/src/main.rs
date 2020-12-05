// https://adventofcode.com/2020/day/6
use std::fs;

fn main() {
    //println!("Day 06\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two());
}

// fn solve_part_one() -> () {
//
// }
//
// fn solve_part_two() -> () {
//
// }

fn read_input(v: &mut String) {
    *v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
}
