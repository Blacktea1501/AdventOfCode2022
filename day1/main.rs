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
    let mut sum: i32 = 0;
    let mut res: i32 = 0;
    for line in vec {
        if line == "" {
            if sum > res {
                res = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    println!("Part1: {}", res);
}

fn part2() {
    let vec = read("in.txt");
    let mut num: i32 = 0;
    let (mut f, mut s, mut t) = (0, 0, 0);
    for line in vec {
        if line == "" {
            if num > f {
                t = s;
                s = f;
                f = num;
            } else if num > s && num != f {
                t = s;
                s = num;
            } else if num > t && num != s {
                t = num;
            }

            num = 0;
        } else {
            num += line.parse::<i32>().unwrap();
        }
    }
    println!("Part2: {}", f + s + t);
}

// file reader i copied from stackoverflow:
// https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
fn read(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("Could not parse line"))
        .collect()
}
