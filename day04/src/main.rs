use std::fs;

fn main() {
    //println!("Day 04\nPart 1: {}\nPart 2: {}", solve_part_one().unwrap(), solve_part_two().unwrap());
}

#[allow(dead_code)]
fn solve_part_one() -> Option<()>{

}

#[allow(dead_code)]
fn solve_part_two() -> Option<()>{

}

fn read_input(v: &mut Vec<Vec<bool>>) {
    let aux: String = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
}