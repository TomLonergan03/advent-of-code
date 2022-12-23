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

    let part_2_top_crate: u32 = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| Result::expect(line, "Line read failure"))
        .map(score_part_2)
        .sum();

    println!("Part 1 score = {part_1_top_crate}");
    println!("Part 2 score = {part_2_top_crate}");
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
            .chars()
            .filter(|x| x != &'[' && x != &']' && x != &' ')
            .collect::<Vec<char>>()
        {
            crates[current_stack].push(item);
            current_stack += 1;
        }
        current_stack = 0;
    }
    return crates;
}

fn score_part_1(moves: Vec<String>, crate_order: Vec<Vec<char>>) -> String {
    for one_move in moves.iter().skip_while(|x| !x.contains("m")) {
        let cleaned_move = one_move
            .chars()
            .filter(|x| x.is_numeric() || x.is_whitespace())
            .collect::<String>()
            .replace("  ", " ")
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse());

        println!("{}", cleaned_move);
    }
    panic!("Not implemented");
}

fn score_part_2(_line: String) -> u32 {
    return 0;
    //panic!("No matching item in group");
}
