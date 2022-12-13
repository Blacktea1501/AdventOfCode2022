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
    let vec = read("in.txt");
    let alph = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut res = 0;
    for line in vec {
        let firsthalf = &line[..line.len() / 2];
        let secondhalf = &line[line.len() / 2..];
        let uc = find_unique_char(firsthalf, secondhalf);
        // since i coudn't figure out how to just add the chars together
        for i in 0..alph.len() {
            if uc == alph.chars().nth(i).unwrap() {
                res += i + 1;
                break;
            }
        }
    }
    println!("Part1: {}", res);
}

fn find_unique_char(firsthalf: &str, secondhalf: &str) -> char {
    let mut common: char = ' ';
    for i in 0..firsthalf.len() {
        for j in 0..firsthalf.len() {
            if firsthalf.chars().nth(i).unwrap() == secondhalf.chars().nth(j).unwrap() {
                common = firsthalf.chars().nth(i).unwrap();
                break;
            }
        }
    }
    common
}

fn part2() {
    let mut vec = read("in.txt");
    let alph = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut res = 0;
    let mut c = ' ';
    for x in 0..100 {
        let first: String = vec.pop().unwrap();
        let sec: String = vec.pop().unwrap();
        let trd: String = vec.pop().unwrap();
        for i in 0..first.len() {
            for j in 0..sec.len() {
                for k in 0..trd.len() {
                    if first.chars().nth(i).unwrap() == sec.chars().nth(j).unwrap()
                        && first.chars().nth(i).unwrap() == trd.chars().nth(k).unwrap()
                    {
                        c = first.chars().nth(i).unwrap();
                    }
                }
            }
        }

        for i in 0..alph.len() {
            if c == alph.chars().nth(i).unwrap() {
                res += i + 1;
                break;
            }
        }
    }
    println!("Part2: {}", res);
}

fn read(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("line could not be parsed"))
        .collect()
}
