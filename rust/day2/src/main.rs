use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("../../inputs/day2.txt");

    let mut current_part_1: u32 = 0;
    let mut current_part_2: u32 = 0;

    let lines = read_lines(path).expect("Line buffer failure");

    for line in lines {
        let line = line.expect("Line read failure");
        current_part_1 += score_part_1(&line);
        current_part_2 += score_part_2(&line);
    }

    println!("Part 1 score = {current_part_1}");
    println!("Part 2 score = {current_part_2}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn score_part_1(text: &String) -> u32 {
    let their_move = text.chars().next().unwrap() as u32 - 64;
    let my_move = text.chars().last().unwrap() as u32 - 87;

    if my_move == 1 && their_move == 3 {
        return 7;
    }
    if my_move == 3 && their_move == 1 {
        return 3;
    }
    if my_move > their_move {
        return my_move + 6;
    }
    if my_move == their_move {
        return my_move + 3;
    }
    if my_move < their_move {
        return my_move;
    }
    panic!("invalid move");
}

fn score_part_2(text: &String) -> u32 {
    let their_move = text.chars().next().unwrap() as u32 - 64;
    let my_result = text.chars().last().unwrap() as u32 - 87;
    if my_result == 1 {
        match their_move {
            1 => return 3,
            2 => return 1,
            3 => return 2,
            _ => panic!("Unexpected move"),
        }
    }
    if my_result == 2 {
        return their_move + 3;
    }
    if my_result == 3 {
        match their_move {
            1 => return 8,
            2 => return 9,
            3 => return 7,
            _ => panic!("Unexpected move"),
        }
    }
    panic!("Invalid result");
}
