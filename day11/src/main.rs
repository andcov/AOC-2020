// https://adventofcode.com/2020/day/11
use std::fs;
use crate::Tile::*;

fn main() {
    println!("Day 11\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two())
}

fn solve_part_one() -> usize{
    let mut room = Vec::new();
    read_input(&mut room);

    let mut adv = advance(&room, false);
    while diff(&room, &adv) {
        room = adv;
        adv = advance(&room, false);
    }

    used_cnt(&room)
}

fn solve_part_two() -> usize{
    let mut room = Vec::new();
    read_input(&mut room);

    let mut adv = advance(&room, true);
    while diff(&room, &adv) {
        room = adv;
        adv = advance(&room, true);
    }

    used_cnt(&room)
}

fn used_cnt(room: &Vec<Vec<Tile>>) -> usize {
    let mut cnt = 0;
    for i in room {
        for j in i {
            if *j == Used {
                cnt += 1;
            }
        }
    }

    cnt
}

fn diff(sf: &Vec<Vec<Tile>>, other: &Vec<Vec<Tile>>) -> bool {
    for i in 0..sf.len() {
        for j in 0..sf[0].len() {
            if sf[i][j] != other[i][j] {
                return true;
            }
        }
    }

    false
}

// 0 for part 1 and 1 for part 2
fn advance(current: &Vec<Vec<Tile>>, part: bool) -> Vec<Vec<Tile>> {
    let mut next = current.clone();

    for i in 1..(current[0].len() - 1) {
        for j in 1..(current.len() - 1) {
            if current[j][i] != Floor {
                let cnt;
                if part {
                    cnt = adjacent_cnt_two(&current, i, j);
                }else {
                    cnt = adjacent_cnt_one(&current, i, j);
                }

                if current[j][i] == Empty && cnt == 0 { next[j][i] = Used }
                else if current[j][i] == Used &&
                    ((cnt >= 5 && part) || (cnt >= 4 && !part)) { next[j][i] = Empty }
            }
        }
    }

    next
}

fn adjacent_cnt_one(room: &Vec<Vec<Tile>>, x: usize, y:usize) -> usize{
    if x == 0 || x == room.len() - 1 ||
        y == 0 || y == room.len() - 1 {
        panic!("Edge case");
    }

    (room[y - 1][x] == Used) as usize + (room[y + 1][x] == Used) as usize +
        (room[y][x + 1] == Used) as usize + (room[y][x - 1] == Used) as usize +
        (room[y - 1][x - 1] == Used) as usize + (room[y - 1][x + 1] == Used) as usize +
        (room[y + 1][x - 1] == Used) as usize + (room[y + 1][x + 1] == Used) as usize
}

fn adjacent_cnt_two(room: &Vec<Vec<Tile>>, x: usize, y:usize) -> usize{
    if x == 0 || x == room[y].len() - 1 ||
        y == 0 || y == room.len() - 1 {
        panic!("Edge case");
    }
    let mut cnt = 0;

    // left
    let mut xx = x;
    while room[y][xx] == Floor || xx == x {
        xx -= 1;
        if xx <= 0 { break }
        if room[y][xx] == Used { cnt += 1; break }
    }

    // right
    xx = x;
    while room[y][xx] == Floor || xx == x {
        xx += 1;
        if xx >= room[y].len() { break }
        if room[y][xx] == Used { cnt += 1; break }
    }

    // down
    let mut yy = y;
    while room[yy][x] == Floor || yy == y {
        yy -= 1;
        if yy <= 0 { break }
        if room[yy][x] == Used { cnt += 1; break }
    }

    // up
    yy = y;
    while room[yy][x] == Floor || yy == y {
        yy += 1;
        if yy >= room.len() { break }
        if room[yy][x] == Used { cnt += 1; break }
    }

    // left down
    xx = x;
    yy = y;
    while room[yy][xx] == Floor || (yy == y && xx == x){
        xx -= 1;
        yy -= 1;
        if yy <= 0 || xx <= 0 { break }
        if room[yy][xx] == Used { cnt += 1; break; }
    }

    // left up
    xx = x;
    yy = y;
    while room[yy][xx] == Floor || (yy == y && xx == x) {
        xx -= 1;
        yy += 1;
        if yy >= room.len() || xx <= 0 { break }
        if room[yy][xx] == Used { cnt += 1; break; }
    }

    // right down
    xx = x;
    yy = y;
    while room[yy][xx] == Floor || (yy == y && xx == x) {
        xx += 1;
        yy -= 1;
        if yy <= 0 || xx >= room[y].len() { break }
        if room[yy][xx] == Used { cnt += 1; break; }
    }

    // right up
    xx = x;
    yy = y;
    while room[yy][xx] == Floor || (yy == y && xx == x) {
        xx += 1;
        yy += 1;
        if yy >= room.len() || xx >= room[y].len() { break }
        if room[yy][xx] == Used { cnt += 1; break; }
    }

    cnt
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Used,
}

fn read_input(room: &mut Vec<Vec<Tile>>) {
    let mut aux = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines().map(|s| {
            let mut v = Vec::new();
            for i in s.chars() {
                if i == 'L' {
                    v.push(Empty);
                } else if i == '.' {
                    v.push(Floor);
                } else if i == '#' {
                    v.push(Used);
                }
            }
            v
        }).collect::<Vec<Vec<Tile>>>();
    let len = aux[0].len();
    let down = (0..len).into_iter().map(|_| Floor).collect::<Vec<Tile>>();
    aux.insert(0, down.clone());
    aux.push(down);
    for i in &mut aux {
        i.insert(0, Floor);
        i.push(Floor);
    }
    *room = aux;
}