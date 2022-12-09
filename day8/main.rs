fn main() {
    part1();
    //part2();
}

#[derive(Debug)]
struct Tree {
    size: usize,
    visible: Option<bool>,
}

fn part1() {
    let mut complete_grid: Vec<Vec<Tree>> = Vec::new();
    for input in std::io::stdin().lines() {
        let mut trees: Vec<Tree> = Vec::new();
        let line = input.unwrap();
        for c in line.chars() {
            trees.push(Tree {
                size: c.to_digit(10).unwrap() as usize,
                visible: None,
            });
        }
        complete_grid.push(trees);
    }

    // initializing all visble trees at the edges
    let len = complete_grid.len();
    let colsize = complete_grid[0].len();
    let mut res = 0;
    for row in 0..len {
        for col in 0..colsize {
            if check_visible(&mut complete_grid, row, col) {
                res += 1;
            }
        }
    }

    println!("Part1: {}", res);

    part2(complete_grid);
}

fn part2(complete_grid: Vec<Vec<Tree>>) {
    let colsize = complete_grid.len();
    let rowsize = complete_grid[0].len();
    let mut all_scores: Vec<usize> = Vec::new();

    for col in 1..colsize - 1 {
        for row in 1..rowsize - 1 {
            all_scores.push(count_scenic_score(&complete_grid, row, col));
        }
    }

    all_scores.sort();
    let len = all_scores.len();
    println!("Part2: {:?}", all_scores[len - 1]);
}

fn count_scenic_score(trees: &Vec<Vec<Tree>>, row: usize, col: usize) -> usize {
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut top: usize = 0;
    let mut bottom: usize = 0;

    let curr_size = trees[col][row].size;

    let colsize = trees.len();
    let rowsize = trees[0].len();

    // left - 1
    for i in (0..row).rev() {
        left += 1;
        if trees[col][i].size >= curr_size {
            break;
        }
    }

    // Right + 1
    for i in (row + 1)..rowsize {
        right += 1;
        if trees[col][i].size >= curr_size {
            break;
        }
    }

    // top - 1
    for i in (0..col).rev() {
        top += 1;
        if trees[i][row].size >= curr_size {
            break;
        }
    }

    //  bottom + 1
    for i in (col + 1)..colsize {
        bottom += 1;
        if trees[i][row].size >= curr_size {
            break;
        }
    }
    right * left * bottom * top
}

// checks if the a tree at a given map is visible, and modifies its visble state
fn check_visible(trees: &mut Vec<Vec<Tree>>, row: usize, col: usize) -> bool {
    // if current_tree has some value the code block is executed
    if let Some(visble) = trees[row][col].visible {
        return visble;
    } else {
        let size = &trees[row][col].size;

        let mut top = true;
        let mut bottom = true;
        let mut left = true;
        let mut right = true;

        // top
        for i in 0..row {
            if size <= &trees[i][col].size {
                top = false;
                break;
            }
        }
        // bottom
        for i in row + 1..trees.len() {
            if size <= &trees[i][col].size {
                bottom = false;
                break;
            }
        }

        // left
        for i in 0..col {
            if size <= &trees[row][i].size {
                left = false;
                break;
            }
        }

        // right
        for i in col + 1..trees[row].len() {
            if size <= &trees[row][i].size {
                right = false;
                break;
            }
        }

        let is_isvisible = left || top || right || bottom;
        trees.get_mut(row).unwrap().get_mut(col).unwrap().visible = Some(is_isvisible);

        is_isvisible
    }
}
