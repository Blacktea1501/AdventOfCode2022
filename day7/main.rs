use std::{collections::HashMap, io};

#[derive(Debug)]
struct F {
    size: usize,
}

fn main() {
    let mut current_d: Vec<String> = Vec::new();
    let mut fullpaths: Vec<String> = Vec::new();
    let mut hm: HashMap<String, usize> = HashMap::new();
    for line in io::stdin().lines() {
        let input = line.unwrap();

        // if input is command
        if input.chars().nth(0).unwrap() == char::from('$') {
            // change directory
            if input.chars().nth(2).unwrap() == char::from('c') {
                if input.chars().nth(5).unwrap() != char::from('.') {
                    let dir = &input[5..input.len()];
                    current_d.push(dir.to_string());
                    let key = format!("{:?}", current_d);
                    fullpaths.push(key.clone());
                    match hm.get(&key) {
                        None => {}
                        Some(prev) => panic!("there's a previous value: {}", prev),
                    };
                    hm.entry(key).or_insert(0);
                } else {
                    fullpaths.pop();
                    current_d.pop();
                }
            }

            // igonring $ ls bc it just gives a list
        }
        // if command ls happend and all the files got listet
        // we just get numbers as file size at the beginning
        if input.chars().nth(0).unwrap().is_digit(10) {
            let file = input
                .split_once(' ')
                .map(|(x, _y)| F {
                    size: x.parse::<usize>().unwrap(),
                })
                .unwrap();

            for fp in &fullpaths {
                *hm.get_mut(fp).unwrap() += file.size;
            }
        }
    }

    // calculating for part1 and part2
    let mut res = 0;
    let mut used_space = 0;
    let max_space = 70_000_000;
    let mut unused_space = 0;
    let mut sizes: Vec<usize> = Vec::new();
    for (directories, fsize) in hm {
        sizes.push(fsize);
        // part1
        if fsize <= 100_000 {
            res += fsize
        }

        if directories == "[\"/\"]" {
            used_space += fsize;
            unused_space = max_space - used_space;
        }
    }

    sizes.sort();
    println!("Part1: {}", res);
    for size in &sizes {
        if unused_space + size >= 30_000_000 {
            println!("Part2: {}", size);
            break;
        }
    }
}
