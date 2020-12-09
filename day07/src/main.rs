// https://adventofcode.com/2020/day/7
use std::fs;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

const SIZE: usize = 600;

fn main() {
    println!("Day 07\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two());
}

fn solve_part_one() -> u32 {
    let search_term = "shiny gold".to_string();
    // MUST BE SQUARE MATRIX
    let mut tree = [[0; SIZE]; SIZE];

    let mut names = HashMap::new();
    let mut _end = 0;

    read_input(&mut tree, &mut names, &mut _end);

    let search_term = *names.get(&search_term).unwrap();
    let mut conn = HashSet::new();
    find_conn(search_term as usize, &mut conn, &tree);

    conn.len() as u32

}

fn solve_part_two() -> u32 {
    let search_term = "shiny gold".to_string();
    // MUST BE SQUARE MATRIX
    let mut tree = [[0; SIZE]; SIZE];

    let mut names = HashMap::new();
    let mut end_index = 0;

    read_input(&mut tree, &mut names, &mut end_index);

    let search_term = *names.get(&search_term).unwrap();
    let mut sums = HashMap::new();

    bags_cnt(search_term as usize, &tree, &mut sums, &end_index)
}

fn bags_cnt(ind: usize, matrix: &[[u32; SIZE]; SIZE], sums: &mut HashMap<u32, u32>, end_index: &u32) -> u32{
    let mut sum = 0;

    for i in 0..matrix[ind].len() {
        if matrix[ind][i] != 0 {
            if i as u32 == *end_index {
                return 0;
            }
            let aux;
            if sums.contains_key(&(i as u32)) {
                aux = *sums.get(&(i as u32)).unwrap();
            } else {
                aux = bags_cnt(i, matrix, sums, end_index);
                sums.insert(i as u32, aux);
            }
            sum = sum + matrix[ind][i] * (aux + 1);
        }
    }

    return sum;
}

fn find_conn(ind: usize, set: &mut HashSet<u32>, matrix: &[[u32; SIZE]; SIZE]) {
    for i in 0..matrix[ind].len() {
        if matrix[i][ind] != 0 {
            set.insert(i as u32);
            find_conn(i, set, matrix);
        }
    }
}

fn add_colour(hm: &mut HashMap<String, u32>, col: &str, name: &mut u32) {
    if !(*hm).contains_key(col) {
        (*hm).insert(col.to_string(), *name);
        *name += 1;
    }
}

fn read_input(_tree: &mut [[u32; SIZE]; SIZE], _names: &mut HashMap<String, u32>, _end_index: &mut u32) {
    let v = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut names = HashMap::new();
    let mut tree = [[0; SIZE]; SIZE];
    let mut cnt = 0;
    let mut end_index = 0;

    let re_bags = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    let re_head = Regex::new(r"(\w+ \w+).+").unwrap();
    let re_no_bags = Regex::new(r"no other bags").unwrap();

    for s in &v {
        let head = re_head.captures(s).unwrap().index(1).to_string();
        add_colour(&mut names, &head, &mut cnt);

        if re_no_bags.is_match(s) {
            if end_index == 0 {
                end_index = cnt;
            }
            add_colour(&mut names, "no other", &mut cnt);

            tree[*names.get(&head).unwrap() as usize]
                [*names.get("no other").unwrap() as usize] = 1;
        } else {
            for col in re_bags.captures_iter(s) {
                add_colour(&mut names, col.index(2), &mut cnt);

                tree[*names.get(&head).unwrap() as usize]
                    [*names.get(col.index(2)).unwrap() as usize] = col.index(1).parse::<u32>().unwrap();
            }
        }
    }
    *_names = names;
    *_tree = tree;
    *_end_index = end_index;
}
