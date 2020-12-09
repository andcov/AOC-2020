// https://adventofcode.com/2020/day/10
use std::fs;

fn main() {
    let mut v = String::new();
    read_input(&mut v);
}

fn read_input(v: &mut String) {
    *v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
}

/*fn main() {
    println!("Day 10\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two());
}

fn solve_part_one() -> u32 {

}

fn solve_part_two() -> u32 {

}*/
