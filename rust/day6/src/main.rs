use core::num;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("../../inputs/day6.txt");
    let raw_text = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| Result::expect(line, "Line read failure"))
        .collect::<String>();

    let part_1_first_marker = find_marker(raw_text.clone(), 4);

    let part_2_first_marker = find_marker(raw_text.clone(), 14);

    println!("Part 1 first marker at = {part_1_first_marker}");
    println!(
        "Part 2 start of message marker at = {}",
        part_2_first_marker
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_marker(mut message: String, number_of_characters: usize) -> usize {
    let mut last_n_chars: Vec<char> = message.clone().drain(..number_of_characters).collect();
    let mut last_n_set: HashSet<char> = message.drain(..number_of_characters).collect();

    let mut index = number_of_characters;

    for character in message.chars() {
        if last_n_set.len() == number_of_characters {
            return index;
        }
        last_n_chars.remove(0);
        last_n_chars.push(character);
        last_n_set.clear();
        for x in 0..number_of_characters {
            last_n_set.insert(last_n_chars[x]);
        }
        index += 1;
    }
    panic!("No marker detected");
}
