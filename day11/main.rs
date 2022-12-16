use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i64>,
    operation: String,
    test: i64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0,
            items: Vec::new(),
            operation: String::new(),
            test: 0,
            if_true: 0,
            if_false: 0,
        }
    }
}


fn main() {
    part1();
    part2();
}

fn part1() {
    let mut monkeys = parse_monkeys();
    let monkeys_len = &monkeys.len();
    let mut inspection_counter = vec![0; *monkeys_len];
    let rounds: usize = 20;
    for _ in 0..rounds {
        for i in 0..*monkeys_len {
            //println!("{:?}", monkeys[i]);
            let items_len = monkeys[i].items.len();
            for _ in 0..items_len {
                let mut item = monkeys[i].items.remove(0);
                inspection_counter[i] += 1;
                item = operation(&monkeys[i].operation, item);
                // divide item by 3 and round down
                item = (item as f64 / 3.0).floor() as i64;
                //println!("Item after operation: {}", item);
                if item % monkeys[i].test == 0 {
                    let if_true = monkeys[i].if_true;
                    monkeys[if_true].items.push(item);
                } else {
                    let if_false = monkeys[i].if_false;
                    monkeys[if_false].items.push(item);
                }
            }
        }
    }

    inspection_counter.sort();
    let max = inspection_counter[inspection_counter.len() - 1];
    let max2 = inspection_counter[inspection_counter.len() - 2];
    println!("Part1: {}", max*max2);
}

fn part2() {
    let mut monkeys = parse_monkeys();
    let monkeys_len = &monkeys.len();
    let mut inspection_counter: Vec<u128> = vec![0; *monkeys_len];
    let rounds: usize = 10_000;
    // least common divider from moneys test
    let lcd = monkeys.iter().map(|m| m.test).product::<i64>();

    for _ in 0..rounds {
        for i in 0..*monkeys_len {

            let items_len = monkeys[i].items.len();
            for _ in 0..items_len {
                let mut item = monkeys[i].items.remove(0);

                inspection_counter[i] += 1;

                item = operation(&monkeys[i].operation, item);

                item = item % lcd;


                if item % monkeys[i].test == 0 {
                    let if_true = monkeys[i].if_true;
                    monkeys[if_true].items.push(item);
                } else {
                    let if_false = monkeys[i].if_false;
                    monkeys[if_false].items.push(item);
                }
            }
        }
    }

    //println!("Inspection: {:?}", inspection_counter);

    inspection_counter.sort();
    let max = inspection_counter[inspection_counter.len() - 1];
    let max2 = inspection_counter[inspection_counter.len() - 2];

    println!("Part2: {}", max*max2);
}


fn operation(operation: &String, item: i64) -> i64 {
    let cond1 = operation.contains("old * o");
    let cond2 = operation.contains("old +");
    let cond3 = operation.contains("old *");

    match (cond1, cond2, cond3) {
        (true, false, true) => item * item,
        (false, true, false) => item + operation[5..].trim().parse::<i64>().unwrap(),
        (false, false, true) => item * operation[5..].trim().parse::<i64>().unwrap(),
        _ => 0,
    }
}

fn parse_monkeys() -> Vec<Monkey> {
    let input = read("in.txt");
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey = Monkey::new();

    for line in &input {
        if line.contains("Monkey") {
            monkey.id = line.chars().nth(7).unwrap().to_digit(10).unwrap() as usize;
            continue;
        }
        if line.contains("Starting items") {
            // remove "Starting items: " from line
            let mod_line = &line[18..];
            // split line into items as i32
            for item in mod_line.split(", ") {
                monkey.items.push(item.parse::<i64>().unwrap());
            }
            continue;
        }
        if line.contains("Operation") {
            // remove "  Operation: " from line
            let mod_line = &line[19..];
            monkey.operation = mod_line.to_string();
            continue;
        }
        if line.contains("Test") {
            // remove "Test: divisible by " from line and strip whitespace
            monkey.test = *&line[20..].trim().parse::<i64>().unwrap();
            continue;
        }
        if line.contains("If true") {
            monkey.if_true = *&line[28..].trim().parse::<usize>().unwrap();
            continue;
        }
        if line.contains("If false") {
            monkey.if_false = *&line[29..].trim().parse::<usize>().unwrap();
            // push monkey to monkeys
            monkeys.push(monkey);
            monkey = Monkey::new();
            continue;
        }
    }
    monkeys
}

fn read(filename: impl AsRef<Path>) -> Vec<String> {
    let f: File = File::open(filename).expect("file not found");
    let buf_reader = BufReader::new(f);
    buf_reader.lines().map(|l| l.expect("Could not parse line")).collect()
}
