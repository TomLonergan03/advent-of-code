use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("../../inputs/day1.txt");
    let mut elves_calories:Vec<u32> = Vec::new();

    let mut current:u32 = 0;

    let lines = read_lines(path).expect("Line buffer failure");
    
    for line in lines {
        let line = line.expect("Line read failure");
        if line == "" {
            elves_calories.push(current);
            current = 0;
            continue;
        }
        current += line.parse::<u32>().expect("line is not number");
    }
    

    elves_calories.sort();
    println!("Highest calorie = {}", elves_calories.last().expect("no array"));
    println!("Top 3 calorie total = {}", elves_calories.iter().rev().take(3).sum::<u32>())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
