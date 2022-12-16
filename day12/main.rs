use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read("in.txt");

    // creating the graph
    let mut graph: Vec<Vec<u8>> = Vec::new();
    for line in input {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            // ignore this ugly peace of code
            match c {
                'a' => row.push(1 as u8),
                'b' => row.push(2 as u8),
                'c' => row.push(3 as u8),
                'd' => row.push(4 as u8),
                'e' => row.push(5 as u8),
                'f' => row.push(6 as u8),
                'g' => row.push(7 as u8),
                'h' => row.push(8 as u8),
                'i' => row.push(9 as u8),
                'j' => row.push(10 as u8),
                'k' => row.push(11 as u8),
                'l' => row.push(12 as u8),
                'm' => row.push(13 as u8),
                'n' => row.push(14 as u8),
                'o' => row.push(15 as u8),
                'p' => row.push(16 as u8),
                'q' => row.push(17 as u8),
                'r' => row.push(18 as u8),
                's' => row.push(19 as u8),
                't' => row.push(20 as u8),
                'u' => row.push(21 as u8),
                'v' => row.push(22 as u8),
                'w' => row.push(23 as u8),
                'x' => row.push(24 as u8),
                'y' => row.push(25 as u8),
                'z' => row.push(26 as u8),
                'E' => row.push(27 as u8),
                _ => row.push(0 as u8),
            }
        }
        graph.push(row);
    }

    // finding starting point
    let mut start_row = 0;
    let mut start_col = 0;
    let row = graph.len();
    let col = graph[0].len();
    for i in 0..row {
        for j in 0..col {
            if graph[i][j] == 0 {
                start_row = i;
                start_col = j;
            }
        }
    }

    // print starting position
    println!("Part1: {}", bfs(graph.clone(), start_row, start_col));
}

fn part2() {
    let input = read("in.txt");
    let mut graph: Vec<Vec<u8>> = Vec::new();
    for line in input {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            // ignore this ugly peace of code.  Here is 'S' 1 and not 0
            match c {
                'a' => row.push(1 as u8),
                'b' => row.push(2 as u8),
                'c' => row.push(3 as u8),
                'd' => row.push(4 as u8),
                'e' => row.push(5 as u8),
                'f' => row.push(6 as u8),
                'g' => row.push(7 as u8),
                'h' => row.push(8 as u8),
                'i' => row.push(9 as u8),
                'j' => row.push(10 as u8),
                'k' => row.push(11 as u8),
                'l' => row.push(12 as u8),
                'm' => row.push(13 as u8),
                'n' => row.push(14 as u8),
                'o' => row.push(15 as u8),
                'p' => row.push(16 as u8),
                'q' => row.push(17 as u8),
                'r' => row.push(18 as u8),
                's' => row.push(19 as u8),
                't' => row.push(20 as u8),
                'u' => row.push(21 as u8),
                'v' => row.push(22 as u8),
                'w' => row.push(23 as u8),
                'x' => row.push(24 as u8),
                'y' => row.push(25 as u8),
                'z' => row.push(26 as u8),
                'E' => row.push(27 as u8),
                _ => row.push(1 as u8),
            }
        }
        graph.push(row);
    }

    // finding starting points
    println!("Part2: {}", bfs2(&graph));
}

fn bfs2(graph: &Vec<Vec<u8>>) -> u32 {
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    let mut distance: Vec<Vec<u32>> = Vec::new();
    let row = graph.len();
    let col = graph[0].len();

    // initialize visited and distance
    for _i in 0..row {
        let mut row_visited: Vec<bool> = Vec::new();
        let mut row_distance: Vec<u32> = Vec::new();
        for _j in 0..col {
            row_visited.push(false);
            row_distance.push(0);
        }
        visited.push(row_visited);
        distance.push(row_distance);
    }

    // push starting point to queue
    let starting_points = find_starting_points(&graph);
    for (row, col) in starting_points {
        queue.push((row, col));
        visited[row][col] = true;
    }

    // BFS
    while !queue.is_empty() {
        let (row, col) = queue.remove(0);
        let current_node = graph[row][col];
        let current_distance = distance[row][col];

        // check if the current node is the end node
        if current_node == 27 {
            return current_distance;
        }

        let neighbors = get_neighbors(&graph, row, col);
        for (r, c) in neighbors {
            if !visited[r][c] && graph[r][c] <= current_node + 1 {
                queue.push((r, c));
                visited[r][c] = true;
                distance[r][c] = current_distance + 1;
            }
        }
    }
    0
}

// all starting points are 0 or 1
fn find_starting_points(graph: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    let row = graph.len();
    let col = graph[0].len();
    for i in 0..row {
        for j in 0..col {
            if graph[i][j] == 1 {
                points.push((i, j));
            }
        }
    }
    points
}

// BFS algorithm to find the shortest path
// but can only move in 4 directions if the next node is max 1 higher
// than the current node and returns the length of the shortest path
fn bfs(graph: Vec<Vec<u8>>, start_row: usize, start_col: usize) -> u32 {
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    let mut distance: Vec<Vec<u32>> = Vec::new();
    let row = graph.len();
    let col = graph[0].len();

    // initialize visited and distance
    for _i in 0..row {
        let mut row_visited: Vec<bool> = Vec::new();
        let mut row_distance: Vec<u32> = Vec::new();
        for _j in 0..col {
            row_visited.push(false);
            row_distance.push(0);
        }
        visited.push(row_visited);
        distance.push(row_distance);
    }

    // push starting point to queue
    queue.push((start_row, start_col));
    visited[start_row][start_col] = true;

    // BFS
    while !queue.is_empty() {
        let (row, col) = queue.remove(0);
        let current_node = graph[row][col];
        let current_distance = distance[row][col];

        // check if the current node is the end node
        if current_node == 27 {
            return current_distance;
        }

        let neighbors = get_neighbors(&graph, row, col);
        for (r, c) in neighbors {
            if !visited[r][c] && graph[r][c] <= current_node + 1 {
                queue.push((r, c));
                visited[r][c] = true;
                distance[r][c] = current_distance + 1;
            }
        }
    }
    0
}

// get neighbors of a node
fn get_neighbors(graph: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let row_len = graph.len();
    let col_len = graph[0].len();

    // up
    if row > 0 {
        if graph[row - 1][col] != 0 {
            neighbors.push((row - 1, col));
        }
    }

    // down
    if row < row_len - 1 {
        if graph[row + 1][col] != 0 {
            neighbors.push((row + 1, col));
        }
    }

    // left
    if col > 0 {
        if graph[row][col - 1] != 0 {
            neighbors.push((row, col - 1));
        }
    }

    // right
    if col < col_len - 1 {
        if graph[row][col + 1] != 0 {
            neighbors.push((row, col + 1));
        }
    }
    neighbors
}

// reader
fn read(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("File not found");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
