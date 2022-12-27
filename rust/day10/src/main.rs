use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Debug)]
enum Operation {
    Noop,
    Addx,
}
fn main() {
    let path = Path::new("../../inputs/day10.txt");
    // let path = Path::new("test.txt");
    let raw_text: Vec<String> = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| line.expect("Line read failure"))
        .collect::<Vec<String>>();

    let (ops, values, cycles): (Vec<Operation>, Vec<i32>, i32) = get_ops(raw_text);

    let part_1_signal_strength = calculate_signal_strength(&ops, &values, cycles, false);

    println!("Part 1 signal strength = {}", part_1_signal_strength);
    println!("Part 2 display");
    _ = calculate_signal_strength(&ops, &values, cycles, true)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_ops(text: Vec<String>) -> (Vec<Operation>, Vec<i32>, i32) {
    let mut ops: Vec<Operation> = vec![];
    let mut values: Vec<i32> = vec![];
    let mut cycles: i32 = 0;
    for mut line in text {
        if line.drain(0..4).collect::<String>() == "addx".to_string() {
            ops.push(Operation::Addx);
            values.push(line.replace(" ", "").parse().expect("No addx value"));
            cycles += 2;
            continue;
        }
        ops.push(Operation::Noop);
        cycles += 1;
    }
    return (ops, values, cycles);
}

fn calculate_signal_strength(
    ops: &Vec<Operation>,
    values: &Vec<i32>,
    cycles: i32,
    render_output: bool,
) -> i32 {
    let mut adding: bool = false;
    let mut total = 0;
    let mut register = 1;
    let mut current_add_index = 0;
    let mut program_counter: i32 = 0;

    for cycle in 1..cycles {
        if render_output {
            render_pixel(cycle, register);
        }
        if cycle % 40 == 20 {
            total += cycle * register;
        }

        if adding {
            register += values[current_add_index];
            adding = false;
            current_add_index += 1;
            continue;
        }

        if ops[program_counter as usize] == Operation::Addx {
            adding = true;
        }
        program_counter += 1
    }
    if render_output {
        print!("\n")
    }
    return total;
}

fn render_pixel(cycle: i32, register: i32) {
    if register - 1 <= (cycle - 1) % 40 && (cycle - 1) % 40 <= register + 1 {
        print!("#");
    } else {
        print!(" ");
    }
    if cycle % 40 == 0 {
        print!("\n");
    }
}
