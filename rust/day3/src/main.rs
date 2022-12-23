use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn main() {
    let path = Path::new("../../inputs/day3.txt");

    let mut current_part_1: u32 = 0;
    let mut current_part_2: u32 = 0;
    let mut group: Vec<String> = vec![];
    let lines = read_lines(path).expect("Line buffer failure");

    for line in lines {
        let line = line.expect("Line read failure");
        current_part_1 += score_part_1(&line);
        group.push(line);
        if group.len() == 3 {
            current_part_2 += score_part_2(&group);
            group.clear();
        }
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

fn score_part_1(line:&String) -> u32 {
    let length = line.chars().count();
    let (compartment_1, compartment_2) = line.split_at(length/2);
    for item_1 in compartment_1.chars(){
        for item_2 in compartment_2.chars(){
            if item_1 != item_2 {
                continue;
            }
            if item_1 < 'a' {
                return item_1 as u32 - 38;
            }
            return  item_1 as u32 - 96;
        }
    }
    panic!("Nothing in rucksack");
}

fn score_part_2(group: &Vec<String>) -> u32 {

    return 0;
}