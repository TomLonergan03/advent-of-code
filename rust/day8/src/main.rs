use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;

fn main() {
    let path = Path::new("../../inputs/day8.txt");
    // let path = Path::new("test.txt");
    let raw_text: Vec<String> = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| Result::expect(line, "Line read failure"))
        .collect::<Vec<String>>();

    let forest = build_forest(raw_text);

    let part_1_visible_trees = total_visible_trees(&forest);

    let part_2_max_score = max_tree_scenic_score(&forest);

    println!("Part 1 number of visible trees = {part_1_visible_trees}");
    println!(
        "Part 2 max scenic score = {}",
        part_2_max_score //tree_scenic_score(&forest, (48, 71))
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn build_forest(input: Vec<String>) -> Vec<Vec<i32>> {
    let mut forest: Vec<Vec<i32>> = vec![vec![]; input.len()];

    for line in input {
        let mut row = 0;
        for tree in line
            .chars()
            .map(|x| x.to_string().parse().expect("Tree isn't int"))
        {
            forest[row].push(tree);
            row += 1;
            // print!("{tree}");
        }
        // print!("\n");
    }
    return forest;
}

fn total_visible_trees(forest: &Vec<Vec<i32>>) -> i32 {
    let mut min_visible_height;
    let mut total_seen = 0;
    let mut trees_seen: Vec<(usize, usize)> = vec![];

    for vertical_start in 0..forest.len() {
        min_visible_height = -1;
        for row in 0..forest.index(0).len() {
            if forest[vertical_start][row] > min_visible_height {
                min_visible_height = forest[vertical_start][row];
                if !trees_seen.contains(&(vertical_start, row)) {
                    total_seen += 1;
                    trees_seen.push((vertical_start, row));
                }
            }
        }
        min_visible_height = -1;
        for row in (0..forest.index(0).len()).rev() {
            if forest[vertical_start][row] > min_visible_height {
                min_visible_height = forest[vertical_start][row];
                if !trees_seen.contains(&(vertical_start, row)) {
                    total_seen += 1;
                    trees_seen.push((vertical_start, row));
                }
            }
        }
    }

    for horizontal_start in 0..forest.index(0).len() {
        min_visible_height = -1;
        for column in 0..forest.len() {
            if forest[column][horizontal_start] > min_visible_height {
                min_visible_height = forest[column][horizontal_start];
                if !trees_seen.contains(&(column, horizontal_start)) {
                    total_seen += 1;
                    trees_seen.push((column, horizontal_start));
                }
            }
        }
        min_visible_height = -1;
        for column in (0..forest.len()).rev() {
            if forest[column][horizontal_start] > min_visible_height {
                min_visible_height = forest[column][horizontal_start];
                if !trees_seen.contains(&(column, horizontal_start)) {
                    total_seen += 1;
                    trees_seen.push((column, horizontal_start));
                }
            }
        }
    }
    return total_seen;
}

fn tree_scenic_score(
    forest: &Vec<Vec<i32>>,
    (horizontal_location, vertical_location): (usize, usize),
) -> i32 {
    if vertical_location == 0
        || horizontal_location == 0
        || vertical_location == forest.index(0).len() - 1
        || horizontal_location == forest.len() - 1
    {
        return 0;
    }
    let max_height = forest[horizontal_location][vertical_location];
    let mut direction_visible = 0;

    //look up
    for vertical_look in (0..vertical_location).rev() {
        direction_visible += 1;
        if forest[horizontal_location][vertical_look] >= max_height {
            break;
        }
    }
    let mut total = direction_visible;
    direction_visible = 0;

    //look down
    for vertical_look in (vertical_location + 1)..forest[horizontal_location].len() {
        direction_visible += 1;
        if forest[horizontal_location][vertical_look] >= max_height {
            break;
        }
    }
    total *= direction_visible;
    direction_visible = 0;

    //look left
    for horizontal_look in (0..horizontal_location).rev() {
        direction_visible += 1;
        if forest[horizontal_look][vertical_location] >= max_height {
            break;
        }
    }
    total *= direction_visible;
    direction_visible = 0;

    //look left
    for horizontal_look in (horizontal_location + 1)..forest.len() {
        direction_visible += 1;
        if forest[horizontal_look][vertical_location] >= max_height {
            break;
        }
    }
    total *= direction_visible;

    // print!("{direction_visible}");
    return total;
}

fn max_tree_scenic_score(forest: &Vec<Vec<i32>>) -> i32 {
    let mut max_score = 0;
    for col in 0..forest.len() {
        for row in 0..forest.index(0).len() {
            let tree_scenic_score = tree_scenic_score(forest, (row, col));
            if tree_scenic_score > max_score {
                max_score = tree_scenic_score;
                //print!("New max {} at ({},{})\n", max_score, col, row);
            }
            // print!("{tree_scenic_score} ");
        }
        // print!("\n");
    }
    return max_score;
}
