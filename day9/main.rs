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

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut positions: Vec<(i32, i32)> = Vec::new();
    positions.push((0, 0));
    for line in input {
        // direction to move
        let d = &line.chars().nth(0).unwrap();

        // steps to move in this direction
        let steps = &line[2..line.len()].parse::<usize>().unwrap();

        make_step(&mut head, &mut tail, *steps, d, &mut positions);
    }
    positions.sort();
    positions.dedup();
    println!("Part1: {}", positions.len());
}

fn make_step<'a>(
    mut head: &'a mut (i32, i32),
    tail: &'a mut (i32, i32),
    steps: usize,
    d: &char,
    pos: &mut Vec<(i32, i32)>,
) {
    for _ in 0..steps {
        let prev = *head;
        if d == &'R' {
            head.0 += 1;
        }
        if d == &'L' {
            head.0 -= 1;
        }
        if d == &'U' {
            head.1 += 1;
        }
        if d == &'D' {
            head.1 -= 1;
        }

        if (head.0 - tail.0).abs() == 2 || (head.1 - tail.1).abs() == 2 {
            *tail = prev;
            pos.push(*tail);
        }
    }
}

fn part2() {
    let input = read("in.txt");
    let mut nods: Vec<(i32, i32)> = Vec::new();

    // initializing nods and the first is the head
    for _ in 0..10 {
        nods.push((0, 0));
    }

    let mut positions: Vec<(i32, i32)> = Vec::new();
    positions.push((0, 0));

    for line in &input {
        // direction to move
        let d = &line.chars().nth(0).unwrap();

        // steps to move in this direction
        let steps = &line[2..line.len()].parse::<usize>().unwrap();
        step2(&mut nods, *steps, d, &mut positions);
    }

    positions.sort();
    positions.dedup();
    let res = positions.len();
    println!("Part2: {}", res);
}

fn step2(nods: &mut Vec<(i32, i32)>, steps: usize, d: &char, pos: &mut Vec<(i32, i32)>) {
    for _ in 0..steps {
        // motion
        if d == &'U' {
            nods[0].1 += 1;
        }
        if d == &'D' {
            nods[0].1 -= 1;
        }
        if d == &'L' {
            nods[0].0 -= 1;
        }
        if d == &'R' {
            nods[0].0 += 1;
        }
        for i in 1..10 {
            nods[i] = movement(nods[i - 1], nods[i]);
            pos.push(nods[9]);
        }
    }
}

fn movement(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let delta_x = (head.0 - tail.0).abs();
    let delta_y = (head.1 - tail.1).abs();

    if std::cmp::max(delta_y, delta_x) <= 1 {
        return tail;
    }

    if head.0 == tail.0 && head.1 > tail.1 {
        // U
        (tail.0, tail.1 + 1)
    } else if head.0 == tail.0 && head.1 < tail.1 {
        // D
        (tail.0, tail.1 - 1)
    } else if head.0 < tail.0 && head.1 == tail.1 {
        // L
        (tail.0 - 1, tail.1)
    } else if head.0 > tail.0 && head.1 == tail.1 {
        // R
        (tail.0 + 1, tail.1)
    } else if head.0 > tail.0 && head.1 > tail.1 {
        // UR
        (tail.0 + 1, tail.1 + 1)
    } else if head.0 > tail.0 && head.1 < tail.1 {
        // UL
        (tail.0 + 1, tail.1 - 1)
    } else if head.0 < tail.0 && head.1 > tail.1 {
        // DR
        (tail.0 - 1, tail.1 + 1)
    } else {
        // DL
        (tail.0 - 1, tail.1 - 1)
    }
}

fn read(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("File does not exist?");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|x| x.expect("Line could not be parsed"))
        .collect()
}
