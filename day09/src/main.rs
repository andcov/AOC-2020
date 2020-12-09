// https://adventofcode.com/2020/day/9
use std::fs;

const FORWARD: usize = 25;

fn main() {
    println!("Day 09\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two());
}

fn solve_part_one() -> i64 {
    let mut v = Vec::new();
    read_input(&mut v);

    return find_problem(&v).unwrap().0;
}

fn solve_part_two() -> i64 {
    let mut v = Vec::new();
    read_input(&mut v);

    if let Some((term, pos)) = find_problem(&v) {
        let mut i = 0;

        while i < pos {
            let mut j = pos - 1;

            while j > i {
                let sum = v[i..j].iter().map(|e| *e).sum::<i64>();

                if sum == term {
                    return *v[i..j].iter().min().unwrap()
                        + *v[i..j].iter().max().unwrap();
                }
                j -= 1;
            }

            i += 1;
        }
    }

    0
}

fn find_problem(v: &Vec<i64>) -> Option<(i64, usize)>{
    for i in 0..(v.len() - FORWARD) {
        let mut first = (v[i..(i + FORWARD)]).to_vec();
        first.sort();

        let mut found = false;
        let search = v[i + FORWARD];
        for j in 0..FORWARD {
            match first.binary_search(&(search - v[i + j])) {
                Ok(_) => { found = true; break; },
                Err(_) => (),
            }
        }

        if !found {
            return Some((v[i + FORWARD], i + FORWARD));
        }
    }

    None
}

fn read_input(v: &mut Vec<i64>) {
    *v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
}
