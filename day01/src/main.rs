use std::fs;

fn main() {
    println!("Day 01\nPart 1: {}\nPart 2: {}", solve_part_one().unwrap(), solve_part_two().unwrap());
}

#[allow(dead_code)]
fn solve_part_one() -> Option<i32> {
    let mut v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split_whitespace().map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    v.sort();

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

#[allow(dead_code)]
fn solve_part_two() -> Option<i32> {
    let mut v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split_whitespace().map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    v.sort();

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