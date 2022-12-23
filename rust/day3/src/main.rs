use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("../../inputs/day3.txt");
    let part_1_priority: u32 = read_lines(path)
        .expect("Line buffer failure")
        .map(Result::unwrap)
        .map(score_part_1)
        .sum();

    let part_2_priority: u32 = read_lines(path)
        .expect("Line buffer failure")
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .chunks(3)
        .map(score_part_2)
        .sum();

    println!("Part 1 score = {part_1_priority}");
    println!("Part 2 score = {part_2_priority}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn determine_priority(item: char) -> u32 {
    if item < 'a' {
        return item as u32 - 38;
    }
    return item as u32 - 96;
}

fn score_part_1(line: String) -> u32 {
    let length = line.chars().count();
    let (compartment_1, compartment_2) = line.split_at(length / 2);
    for item_1 in compartment_1.chars() {
        for item_2 in compartment_2.chars() {
            if item_1 != item_2 {
                continue;
            }
            return determine_priority(item_1);
        }
    }
    panic!("Nothing in rucksack");
}

fn score_part_2(group: &[String]) -> u32 {
    for item_0 in group[0].chars() {
        for item_1 in group[1].chars() {
            for item_2 in group[2].chars() {
                if item_0 != item_1 {
                    break;
                }
                if item_1 != item_2 {
                    continue;
                }
                return determine_priority(item_1);
            }
        }
    }
    panic!("No matching item in group");
}
