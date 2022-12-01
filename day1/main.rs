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
    let mut sum: i32 = 0;
    let mut numbers: Vec<i32> = Vec::new();
    for line in vec {
        if line == "" {
            numbers.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    let (mut fst, mut snd, mut trd) = (0, 0, 0);
    // getting the 3 largest numbers
    for n in numbers {
        if n > fst {
            trd = snd;
            snd = fst;
            fst = n;
        } else if n > snd && n != fst {
            trd = snd;
            snd = n;
        } else if n > trd && n != snd {
            trd = n;
        }
    }

    println!("Part2: {}", fst + snd + trd);
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
