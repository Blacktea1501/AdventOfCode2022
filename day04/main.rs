use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    part1();
    part2()
}

fn part1() {
    let vec = read("in.txt");
    let mut cnt = 0;
    for line in vec {
        let x = line.split_once(",").unwrap();
        let a = x.0.split_once("-").unwrap();
        let b = x.1.split_once("-").unwrap();
        let a0 = a.0.parse::<i32>().unwrap();
        let a1 = a.1.parse::<i32>().unwrap();
        let b0 = b.0.parse::<i32>().unwrap();
        let b1 = b.1.parse::<i32>().unwrap();

        if (a0 <= b0 && a1 >= b1) || (a0 >= b0 && a1 <= b1) {
            cnt += 1;
        }
    }
    println!("Part1: {}", cnt);
}

fn part2() {
    let vec = read("in.txt");
    let mut cnt = 0;
    for line in vec {
        let x = line.split_once(",").unwrap();
        let a = x.0.split_once("-").unwrap();
        let b = x.1.split_once("-").unwrap();
        let a0 = a.0.parse::<i32>().unwrap();
        let a1 = a.1.parse::<i32>().unwrap();
        let b0 = b.0.parse::<i32>().unwrap();
        let b1 = b.1.parse::<i32>().unwrap();

        if (a1 >= b0 && a1 <= b1) || (b1 >= a0 && b1 <= a1) {
            cnt += 1;
        }
    }
    println!("Part2: {}", cnt);
}

fn read(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("Line could not be parsed!"))
        .collect()
}
