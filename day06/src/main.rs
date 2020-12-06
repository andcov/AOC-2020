// https://adventofcode.com/2020/day/6
use std::fs;
use regex::Regex;
use std::collections::{HashSet, HashMap};

fn main() {
    println!("Day 06\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two());
}

fn solve_part_one() -> u32 {
    let mut v = Vec::new();
    read_input(&mut v);

    let mut cnt: u32 = 0;

    for i in &v {
        let mut ans = HashSet::new();
        for j in i.lines() {
            for c in j.chars() {
                ans.insert(c);
            }
        }
        cnt += ans.len() as u32;
    }

    cnt
}

fn solve_part_two() -> u32 {
    let mut v = Vec::new();
    read_input(&mut v);

    let mut cnt: u32 = 0;

    for i in &v {
        let mut ans = HashMap::new();
        let mut lines_cnt: u32 = 0;

        for j in i.lines() {
            lines_cnt += 1;
            for c in j.chars() {
                if ans.contains_key(&c) {
                    let aux = ans.get_mut(&c).unwrap();
                    *aux += 1;
                } else {
                    ans.insert(c, 1u32);
                }
            }
        }
        ans.iter().for_each(|(_, u)| {
            if *u == lines_cnt {
                cnt += 1;
            }
        });
    }

    cnt
}

fn read_input(v: &mut Vec<String>) {
    let aux = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let aux = Regex::new(r"\n\s").unwrap()
        .split(&aux)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    *v = aux;
}
