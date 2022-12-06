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
    let mut cnt = 4;
    let size: usize = 4;
    for line in input {
        for c in line.chars().collect::<Vec<char>>().windows(size) {
            if all_distinct(c, size) {
                println!("Part1: {}", cnt);
                break;
            } else {
                cnt += 1;
            }
        }
    }
}

fn part2() {
    let input = read("in.txt");
    let mut cnt = 14;
    let size: usize = 14;
    for line in input {
        for c in line.chars().collect::<Vec<char>>().windows(size) {
            if all_distinct(c, size) {
                println!("Part2: {}", cnt);
                break;
            } else {
                cnt += 1;
            }
        }
    }
}

fn all_distinct(chars: &[char], len: usize) -> bool {
    let mut chars = chars.to_vec();
    chars.sort();
    chars.dedup(); // removes duplicates
    chars.len() == len
}

fn read(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such File!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("Line could not be parsed!"))
        .collect()
}
