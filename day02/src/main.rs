use std::fs;
use regex::Regex;
use std::str::FromStr;

fn main() {
    println!("Day 02\nPart 1: {}\nPart 2: {}", solve_part_one().unwrap(), solve_part_two().unwrap());
}

fn solve_part_one() -> Option<i32> {
    let v = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let re = Regex::new(r"[- :]+").unwrap();
    let mut valid_pass = 0;

    for l in v.lines() {
        let f: Vec<&str> = re.split(l).collect();
        let r = Regex::from_str(f[2]).unwrap();

        let mut cnt = 0;
        r.find_iter(f[3]).for_each(|_| cnt += 1);

        if cnt >= f[0].parse().unwrap() && cnt <= f[1].parse().unwrap() {
            valid_pass += 1;
        }
    }

    Some(valid_pass)
}

fn solve_part_two() -> Option<i32> {
    let v = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
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
