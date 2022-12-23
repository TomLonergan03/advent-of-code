use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("../../inputs/day4.txt");
    let part_1_priority: u32 = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| Result::expect(line, "Line read failure"))
        .map(score_part_1)
        .sum();

    let part_2_priority: u32 = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| Result::expect(line, "Line read failure"))
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

fn score_part_1(line: String) -> u32 {
    let sections: Vec<&str> = line.split(",").collect();
    let section1: Vec<&str> = sections[0].split("-").collect();
    let section1_start: u32 = section1[0].parse().expect("Section id not a number");
    let section1_end: u32 = section1[1].parse().expect("Section id not a number");

    let section2: Vec<&str> = sections[1].split("-").collect();
    let section2_start: u32 = section2[0].parse().expect("Section id not a number");
    let section2_end: u32 = section2[1].parse().expect("Section id not a number");

    if (section1_start <= section2_start && section1_end >= section2_end)
        || (section1_start >= section2_start && section1_end <= section2_end)
    {
        return 1;
    }
    return 0;
}

fn score_part_2(line: String) -> u32 {
    let sections: Vec<&str> = line.split(",").collect();
    let section1: Vec<&str> = sections[0].split("-").collect();
    let section1_start: u32 = section1[0].parse().expect("Section id not a number");
    let section1_end: u32 = section1[1].parse().expect("Section id not a number");

    let section2: Vec<&str> = sections[1].split("-").collect();
    let section2_start: u32 = section2[0].parse().expect("Section id not a number");
    let section2_end: u32 = section2[1].parse().expect("Section id not a number");

    if (section1_end >= section2_start && section1_end <= section2_end)
        || (section1_start <= section2_end && section1_start >= section2_start)
        || (section1_start <= section2_start && section1_end >= section2_end)
        || (section1_start >= section2_start && section1_end <= section2_end)
    {
        return 1;
    }
    return 0;
    //panic!("No matching item in group");
}
