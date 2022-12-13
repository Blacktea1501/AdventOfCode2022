use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    part1();
    part2();
}

fn read(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("Could not parse line"))
        .collect()
}

fn part1() {
    let input = read("in.txt");
    let mut res: i32 = 0;
    for line in input {
        let l = line.split_once(' ').unwrap();
        if l.0 == "A" && l.1 == "Y" {
            res += 8;
        } else if l.0 == "B" && l.1 == "Z" {
            res += 9;
        } else if l.0 == "C" && l.1 == "X" {
            res += 7;
        } else if l.0 == "A" && l.1 == "X" {
            res += 4;
        } else if l.0 == "B" && l.1 == "Y" {
            res += 5;
        } else if l.0 == "C" && l.1 == "Z" {
            res += 6;
        } else if l.0 == "A" && l.1 == "Z" {
            res += 3;
        } else if l.0 == "B" && l.1 == "X" {
            res += 1;
        } else if l.0 == "C" && l.1 == "Y" {
            res += 2;
        }
    }
    println!("Part1: {}", res);
}

fn part2() {
    let input = read("in.txt");
    let mut res: i32 = 0;
    for line in input {
        let l = line.split_once(' ').unwrap();
        if l.0 == "A" && l.1 == "Y" {
            res += 4;
        } else if l.0 == "B" && l.1 == "Z" {
            res += 9;
        } else if l.0 == "C" && l.1 == "X" {
            res += 2;
        } else if l.0 == "A" && l.1 == "X" {
            res += 3;
        } else if l.0 == "B" && l.1 == "Y" {
            res += 5;
        } else if l.0 == "C" && l.1 == "Z" {
            res += 7;
        } else if l.0 == "A" && l.1 == "Z" {
            res += 8;
        } else if l.0 == "B" && l.1 == "X" {
            res += 1;
        } else if l.0 == "C" && l.1 == "Y" {
            res += 6;
        }
    }
    println!("Part2: {}", res);
}
