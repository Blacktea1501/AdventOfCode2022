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
    let file = File::open(filename).expect("Could not open file!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("could not parse line"))
        .collect()
}

fn part1() {
    let input = read("in.txt");
    let mut s = [
        vec!['B', 'P', 'N', 'Q', 'H', 'D', 'R', 'T'],
        vec!['W', 'G', 'B', 'J', 'T', 'V'],
        vec!['N', 'R', 'H', 'D', 'S', 'V', 'M', 'Q'],
        vec!['P', 'Z', 'N', 'M', 'C'],
        vec!['D', 'Z', 'B'],
        vec!['V', 'C', 'W', 'Z'],
        vec!['G', 'Z', 'N', 'C', 'V', 'Q', 'L', 'S'],
        vec!['L', 'G', 'J', 'M', 'D', 'N', 'V'],
        vec!['T', 'P', 'M', 'F', 'Z', 'C', 'G'],
    ];

    let mut res = String::new();
    for line in input {
        let instructions: Vec<usize> = modify_string(line);
        for _ in 0..instructions[0] {
            if !s[instructions[1] - 1].is_empty() {
                let cargo = s[instructions[1] - 1].pop().unwrap();
                s[instructions[2] - 1].push(cargo);
            }
        }
    }
    for i in 0..9 {
        if !s[i].is_empty() {
            res.push(s[i].pop().unwrap());
        }
    }
    println!("Part1: {}", res);
}

fn part2() {
    let input = read("in.txt");
    let mut s: Vec<Vec<char>> = vec![
        vec!['B', 'P', 'N', 'Q', 'H', 'D', 'R', 'T'],
        vec!['W', 'G', 'B', 'J', 'T', 'V'],
        vec!['N', 'R', 'H', 'D', 'S', 'V', 'M', 'Q'],
        vec!['P', 'Z', 'N', 'M', 'C'],
        vec!['D', 'Z', 'B'],
        vec!['V', 'C', 'W', 'Z'],
        vec!['G', 'Z', 'N', 'C', 'V', 'Q', 'L', 'S'],
        vec!['L', 'G', 'J', 'M', 'D', 'N', 'V'],
        vec!['T', 'P', 'M', 'F', 'Z', 'C', 'G'],
    ];

    let mut res = String::new();

    for line in input {
        let instructions: Vec<usize> = modify_string(line);
        let mut crates = Vec::new();
        for _ in 0..instructions[0] {
            if !s[instructions[1] - 1].is_empty() {
                crates.push(s[instructions[1] - 1].pop().unwrap());
            }
        }
        for _ in 0..instructions[0] {
            if !crates.is_empty() {
                s[instructions[2] - 1].push(crates.pop().unwrap());
            }
        }
    }

    for i in 0..9 {
        if !s[i].is_empty() {
            res.push(s[i].pop().unwrap());
        }
    }

    println!("Part2: {}", res);
}

fn modify_string(s: String) -> Vec<usize> {
    let binding = s
        .replace("move ", " ")
        .replace(" from ", " ")
        .replace(" to ", " ");
    binding
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}
