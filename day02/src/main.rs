use std::fs;
use regex::Regex;

fn main() {
    println!("Day 02\nPart 1: {}\nPart 2: {}", solve_part_one().unwrap(), solve_part_two().unwrap());
}

fn solve_part_one() -> Option<i32> {
    let mut v = String::new();
    read_input(&mut v);

    let re = Regex::new(r"[- :]+").unwrap();
    let mut valid_pass = 0;

    for l in v.lines() {
        let f: Vec<&str> = re.split(l).collect();
        let cnt = f[3].matches(f[2]).count();

        if cnt >= f[0].parse().unwrap() && cnt <= f[1].parse().unwrap() {
            valid_pass += 1;
        }
    }

    Some(valid_pass)
}

fn solve_part_two() -> Option<i32> {
    let mut v = String::new();
    read_input(&mut v);

    let re = Regex::new(r"[- :]+").unwrap();
    let mut valid_pass = 0;

    for l in v.lines() {
        let f: Vec<&str> = re.split(l).collect();

        if f[3].chars().nth((f[0].parse::<i32>().unwrap() - 1) as usize).unwrap().eq(&f[2].chars().nth(0).unwrap()) ^
            f[3].chars().nth((f[1].parse::<i32>().unwrap() - 1) as usize).unwrap().eq(&f[2].chars().nth(0).unwrap()){
            valid_pass += 1;
        }
    }

    Some(valid_pass)
}

fn read_input(v: &mut String) {
    *v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
}
