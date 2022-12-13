use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read("in.txt");
    let mut register_x: i32 = 1;
    let mut cycle_number: i32 = 0;
    let mut signals: Vec<i32> = Vec::new();
    for line in input {
        if line.len() == 4 {
            // noop

            cycle_number += 1;
            calc(&cycle_number, &register_x, &mut signals);
        } else {
            // addx
            cycle_number += 1;
            calc(&cycle_number, &register_x, &mut signals);
            cycle_number += 1;
            register_x += modify_string(&line);
            calc(&cycle_number, &register_x, &mut signals);
        }
    }
    let mut res = 0;
    for i in signals {
        res += i;
    }
    println!("Part1: {}", res);
}

fn part2() {
    let input = read("in.txt");
    let mut register_x: i32 = 1;
    let mut cycle_number: i32 = 0;
    println!("Part2: ");
    for line in input {
        if line.len() == 4 {
            // noop
            draw(&cycle_number, &register_x);
            cycle_number += 1;
        } else {
            // addx
            draw(&cycle_number, &register_x);
            cycle_number += 1;
            draw(&cycle_number, &register_x);
            cycle_number += 1;
            register_x += modify_string(&line);
        }
    }
}

fn draw(cn: &i32, reg_x: &i32) {
    if cn % 40 == 0 {
        println!();
    }

    if ((cn % 40) - reg_x).abs() <= 1 {
        print!("■");
    } else {
        print!(" ");
    }
}

fn modify_string(line: &String) -> i32 {
    let split = line.split_whitespace();
    split.last().unwrap().to_string().parse::<i32>().unwrap()
}

fn calc(cn: &i32, reg_x: &i32, sig: &mut Vec<i32>) {
    if cn == &20 || cn == &60 || cn == &100 || cn == &140 || cn == &180 || cn == &220 {
        sig.push(reg_x * cn);
    }
}

fn read(filename: impl AsRef<Path>) -> Vec<String> {
    let f: File = File::open(filename).expect("File does not exist!");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not read line."))
        .collect()
}
