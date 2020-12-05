// https://adventofcode.com/2020/day/5
use std::fs;

fn main() {
    println!("Day 05\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two());

}

fn get_row(x: &str) -> u32 {
    let mut max: u32 = 127;
    let mut min: u32 = 0;

    for i in x.chars(){
        let mij = (max + min) / 2;
        if i == 'F' {
            max = mij;
        } else if i == 'B' {
            min = mij + 1;
        }
    }

    min
}

fn get_column(x: &str) -> u32{
    let mut max: u32 = 7;
    let mut min: u32 = 0;

    for i in x.chars(){
        let mij = (max + min) / 2;
        if i == 'L' {
            max = mij;
        } else if i == 'R' {
            min = mij + 1;
        }
    }

    min
}

fn solve_part_one() ->  u32{
    let mut v = String::new();
    read_input(&mut v);

    let mut max = 0;

    for i in v.lines() {
        let id = get_row(&i[..7]) * 8 + get_column(&i[7..]);

        if id > max {
            max = id;
        }
    }

    max
}

fn solve_part_two() -> u32 {
    let mut v = String::new();
    read_input(&mut v);

    let mut plane = [false; 1024];

    for i in v.lines() {
        plane[(8 * get_row(&i[..7]) + get_column(&i[7..])) as usize] = true;
    }

    for i in 1..1023 {
        if !plane[i] && (plane[i - 1] && plane[i + 1]) {
            return i as u32;
        }
    }

    0
}

fn read_input(v: &mut String) {
    *v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
}