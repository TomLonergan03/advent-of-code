use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("../../inputs/day5.txt");
    let raw_text = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| Result::expect(line, "Line read failure"))
        .collect::<Vec<String>>();

    let initial_crate_order = parse_crate_order(raw_text.clone());

    let part_1_top_crate: String = score_part_1(raw_text.clone(), initial_crate_order.clone());

    let part_2_top_crate: String = score_part_2(raw_text.clone(), initial_crate_order.clone());

    println!("Part 1 order = {part_1_top_crate}");
    println!("Part 2 order = {part_2_top_crate}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_crate_order(setup: Vec<String>) -> Vec<Vec<char>> {
    let mut initial_order: Vec<&String> = setup
        .iter()
        .rev()
        .skip_while(|x| x.contains("m"))
        .skip(1)
        .collect();

    let number_of_stacks: u32 = initial_order
        .remove(0)
        .clone()
        .trim()
        .split_whitespace()
        .last()
        .expect("No stack number")
        .parse()
        .expect("Largest stack not a number");

    let mut crates: Vec<Vec<char>> = vec![];

    for _ in 0..number_of_stacks {
        crates.push(vec![]);
    }

    for line in initial_order {
        let mut current_stack = 0;
        for item in line
            .replace("    ", "-")
            .replace("[", "")
            .replace("]", ",")
            .replace(" ", "")
            .split(",")
            .collect::<String>()
            .chars()
        {
            if item != '-' {
                crates[current_stack].push(item);
            }
            current_stack += 1;
        }
    }
    return crates;
}

fn clean_move(unclean_move: &String) -> Vec<i32> {
    return unclean_move
        .chars()
        .filter(|x| x.is_numeric() || x.is_whitespace())
        .collect::<String>()
        .replace("  ", " ")
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().expect("Move not a number"))
        .collect::<Vec<i32>>();
}

fn score_part_1(moves: Vec<String>, mut crate_order: Vec<Vec<char>>) -> String {
    for one_move in moves.iter().skip_while(|x| !x.contains("m")) {
        let cleaned_move: Vec<i32> = clean_move(one_move);

        for _ in 0..cleaned_move[0] {
            let moving_crate = crate_order[cleaned_move[1] as usize - 1]
                .pop()
                .expect("No crate to be moved");
            crate_order[cleaned_move[2] as usize - 1].push(moving_crate);
        }
    }
    let mut top_row: String = String::new();
    for stack in crate_order {
        top_row.push(stack.last().expect("Empty stack").to_owned());
    }
    return top_row;
}

fn score_part_2(moves: Vec<String>, mut crate_order: Vec<Vec<char>>) -> String {
    for one_move in moves.iter().skip_while(|x| !x.contains("m")) {
        let cleaned_move: Vec<i32> = clean_move(one_move);

        let mut moving_crates: Vec<char> = vec![];

        for _ in 0..cleaned_move[0] {
            moving_crates.push(
                crate_order[cleaned_move[1] as usize - 1]
                    .pop()
                    .expect("No crate to be moved"),
            )
        }

        for _ in 0..cleaned_move[0] {
            crate_order[cleaned_move[2] as usize - 1]
                .push(moving_crates.pop().expect("No crate to be moved"))
        }
    }
    let mut top_row: String = String::new();
    for stack in crate_order {
        top_row.push(stack.last().expect("Empty stack").to_owned());
    }
    return top_row;
}
