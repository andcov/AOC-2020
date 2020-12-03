use std::fs;

fn main() {
    println!("Day 03\nPart 1: {}\nPart 2: {}", solve_part_one().unwrap(), solve_part_two().unwrap());
}

fn solve_part_one() -> Option<u64>{
    let mut v = Vec::new();
    read_input(&mut v);

    Some(trees_encountered(&v, 3, 1))
}

fn solve_part_two() -> Option<u64>{
    let mut v = Vec::new();
    read_input(&mut v);

    let prod: u64 = (1..8).into_iter().filter(|i| *i % 2 != 0)
        .map(|j| trees_encountered(&v, j, 1))
        .product::<u64>();

    Some(prod * trees_encountered(&v, 1, 2))
}

fn trees_encountered(map: &Vec<Vec<bool>>, right: usize, down: usize) -> u64 {
    let width = map[0].len();
    let height = map.len();

    let mut x = 0;
    let mut y = 0;
    let mut cnt = 0;

    while y < height - 1 {
        x += right;
        if x >= width { x -= width };
        y += down;

        if map[y][x] {
            cnt += 1;
        }
    }

    cnt
}

fn read_input(v: &mut Vec<Vec<bool>>) {
    let aux: String = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    *v = aux.split("\n").map(|i| i.chars().map(|j| {
        if j == '.' {
            return false;
        }else {
            return true;
        }
    }).collect::<Vec<bool>>()).collect();
}