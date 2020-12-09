// https://adventofcode.com/2020/day/8
use std::fs;
use crate::Instr::*;

fn main() {
    println!("Day 08\nPart 1: {}\nPart 2: {}", solve_part_one(), solve_part_two());

}

#[derive(Copy, Clone)]
enum Instr {
    Nop,
    Acc(i32),
    Jmp(i32),
}

fn is_cycle(x: &Vec<Instr>) -> (bool, i32){
    let length = x.len();
    let mut instr = x.iter()
        .map(|i| *i)
        .zip((0..length).map(|_i| false))
        .collect::<Vec<(Instr, bool)>>();

    let mut i = 0;
    let mut acc = 0;
    while (i as usize) < instr.len() {
        if instr[i as usize].1 {
            return (true, acc);
        }
        instr[i as usize].1 = true;

        match instr[i as usize].0 {
            Nop => i += 1,
            Acc(pl) => { acc += pl; i += 1},
            Jmp(x) => i += x,
        }
    }

    (false, acc)
}

fn read_input(v: &mut Vec<Instr>) {
    let aux = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut instr = Vec::new();

    for i in aux.lines() {
        let iter = i.split(" ").collect::<Vec<&str>>();
        let name = iter[0];
        let number = iter[1].parse::<i32>().unwrap();

        if name.eq("nop") {
            instr.push(Instr::Nop);
        } else if name.eq("acc") {
            instr.push(Instr::Acc(number));
        } else {
            instr.push(Instr::Jmp(number));
        }
    }

    *v = instr;
}

fn solve_part_one() -> i32 {
    let mut instr = Vec::new();
    read_input(&mut instr);
    return is_cycle(& instr).1;
}

fn solve_part_two() -> i32 {
    let mut instr = Vec::new();
    read_input(&mut instr);

    let length = instr.len();
    let mut instr_ex = instr.iter()
        .map(|i| *i)
        .zip((0..length).map(|_i| false))
        .collect::<Vec<(Instr, bool)>>();


    let mut i = 0;
    while (i as usize) < instr.len() {
        if instr_ex[i as usize].1 {
            break;
        }

        instr_ex[i as usize].1 = true;
        match instr_ex[i as usize].0 {
            Jmp(x) => i += x,
            _ => i += 1,
        }
    }

    for i in 0..instr.len() {
        if instr_ex[i].1 {
            match instr[i] {
                Nop => {
                    let mut j = i + 1;
                    while j > 0 {
                        j -= 1;

                        if !instr_ex[j].1 {
                            let temp = instr[i];
                            instr[i] = Jmp(i as i32 - j as i32);
                            let aux = is_cycle(&instr);
                            if aux.0 == false {
                                return aux.1;
                            }
                            instr[i] = temp;
                        }
                    }
                    j = i;
                    while j < instr.len() {
                        if !instr_ex[j].1 {
                            let temp = instr[i];
                            instr[i] = Jmp(j as i32 - i as i32);
                            let aux = is_cycle(&instr);
                            if aux.0 == false {
                                return aux.1;
                            }
                            instr[i] = temp;
                        }
                        j += 1;
                    }
                },
                Jmp(_) => {
                    if (i + 1 < instr.len() && !instr_ex[i + 1].1) || i + 2 == instr.len() {
                        let temp = instr[i];
                        instr[i] = Nop;
                        let aux = is_cycle(&instr);
                        if aux.0 == false {
                            return aux.1;
                        }
                        instr[i] = temp;
                    }
                },

                _ => (),
            }
        }
    }

    0
}
