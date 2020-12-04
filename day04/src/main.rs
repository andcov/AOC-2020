use std::fs;
use regex::Regex;
use fancy_regex;

fn main() {
    println!("Day 04\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two());
}

fn solve_part_one() -> u32 {
    let mut v = Vec::new();
    read_input(&mut v);

    let mut cnt: u32 = 0;

    for i in v {
        if is_valid(&i) {
            cnt += 1;
        }
    }

    cnt
}

fn solve_part_two() -> u32 {
    let mut v = Vec::new();
    read_input(&mut v);

    let mut cnt: u32 = 0;

    for i in v {
        if is_valid(&i) {
            let aux = format!("{} ", i);
            if is_byr_valid(&aux) && is_iyr_valid(&aux) && is_eyr_valid(&aux) &&
                is_hgt_valid(&aux) && is_hcl_valid(&aux) && is_ecl_valid(&aux) && is_pid_valid(&aux) {
                cnt += 1;
            }
        }
    }

    cnt
}

fn is_valid(x: &str) -> bool {
    let r = fancy_regex::Regex::new(r"(?=.*byr)(?=.*iyr)(?=.*eyr)(?=.*hgt)(?=.*hcl)(?=.*ecl)(?=.*pid).*").unwrap();

    if r.is_match(x).unwrap() {
        return true;
    }

    false
}

fn is_byr_valid(s: &str) -> bool {
    let r = Regex::new(r"byr:(\d{4})\s").unwrap();

    let caps = match r.captures(s) {
        None => { return false }
        Some(a) => { a }
    };

    let byr = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();

    byr >= 1920 && byr <= 2002
}

fn is_iyr_valid(s: &str) -> bool {
    let r = Regex::new(r"iyr:(\d{4})\s").unwrap();

    let caps = match r.captures(s) {
        None => { return false }
        Some(a) => { a }
    };

    let iyr = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();

    iyr >= 2010 && iyr <= 2020
}

fn is_eyr_valid(s: &str) -> bool {
    let r = Regex::new(r"eyr:(\d{4})\s").unwrap();

    let caps = match r.captures(s) {
        None => { return false }
        Some(a) => { a }
    };

    let eyr = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();

    eyr >= 2020 && eyr <= 2030
}

fn is_hgt_valid(s: &str) -> bool {
    let r = Regex::new(r"hgt:(\d+)(cm|in)\s").unwrap();

    let caps = match r.captures(s) {
        None => { return false }
        Some(a) => { a }
    };

    let hgt = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let unit = caps.get(2).unwrap().as_str();

    ( unit.eq("cm") && hgt >= 150 && hgt <= 193 ) ||
        ( unit.eq("in") && hgt >= 59 && hgt <= 76 )
}

fn is_hcl_valid(s: &str) -> bool {
    let r = Regex::new(r"hcl:#([\da-f]{6})\s").unwrap();

    return match r.captures(s) {
        None => { false }
        Some(_) => { true }
    };
}

fn is_ecl_valid(s: &str) -> bool {
    let r = Regex::new(r"ecl:(blu|brn|amb|gry|grn|hzl|oth)\s").unwrap();

    return match r.captures(s) {
        None => { false }
        Some(_) => { true }
    }
}

fn is_pid_valid(s: &str) -> bool {
    let r = Regex::new(r"pid:(\d{9})\s").unwrap();

    match r.captures(s) {
        None => { false }
        Some(_) => { true }
    }
}

fn read_input(v: &mut Vec<String>) {
    let aux = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let aux = Regex::new(r"\n\s").unwrap()
        .split(&aux)
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.replace("\n", " "))
        .collect::<Vec<String>>();
    *v = aux;
}