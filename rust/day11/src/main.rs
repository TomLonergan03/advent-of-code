use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Clone)]
enum Operation {
    Add,
    Mul,
    Square,
}
#[derive(Clone)]
struct Monkey {
    items: Vec<i128>,
    operator: Operation,
    operator_value: i128,
    test_divisor: i128,
    test_true_target: usize,
    test_false_target: usize,
    total_items_inspected: i128,
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            items: vec![],
            operator: Operation::Add,
            operator_value: 0,
            test_divisor: 0,
            test_true_target: 0,
            test_false_target: 0,
            total_items_inspected: 0,
        }
    }
    pub fn set_items(&mut self, items: Vec<i128>) {
        self.items = items;
    }
    pub fn set_operator(&mut self, operator: Operation) {
        self.operator = operator;
    }
    pub fn set_operator_value(&mut self, value: i128) {
        self.operator_value = value;
    }
    pub fn set_test_divisor(&mut self, divisor: i128) {
        self.test_divisor = divisor;
    }
    pub fn set_true_target(&mut self, target: usize) {
        self.test_true_target = target;
    }
    pub fn set_false_target(&mut self, target: usize) {
        self.test_false_target = target;
    }
    pub fn add_item(&mut self, item: i128) {
        self.items.push(item);
    }
    // returns vec of tuple (target, item)
    pub fn update(&mut self, worry_levels_divide: bool) -> Vec<(usize, i128)> {
        let mut throws: Vec<(usize, i128)> = vec![];
        for _ in 0..self.items.len() {
            let mut current_item = self.items.remove(0);
            // println!("  Monkey inspects item with worry level of {current_item}");
            if self.operator == Operation::Square {
                current_item = current_item * current_item;
                // println!("   Worry level is squared to {current_item}");
            }
            if self.operator == Operation::Mul {
                current_item = current_item * self.operator_value;
                // println!(
                //     "   Worry level is multiplied by {} to {current_item}",
                //     self.operator_value
                // );
            }
            if self.operator == Operation::Add {
                current_item = current_item + self.operator_value;
                // println!(
                //     "   Worry level increases by {} to {current_item}",
                //     self.operator_value
                // );
            }
            if worry_levels_divide {
                current_item = current_item / 3;
            }
            self.total_items_inspected += 1;
            // println!(
            //     "   Monkey gets bored with item. Worry level is divided by 3 to {current_item}"
            // );
            if current_item % self.test_divisor as i128 == 0 {
                throws.push((self.test_true_target, current_item));
                // println!(
                //     "   Current worry level is divisible by {}.",
                //     self.test_divisor
                // );
                // println!(
                //     "   Item with worry level {current_item} is thrown to monkey {}.",
                //     self.test_true_target
                // )
            }
            if current_item % self.test_divisor as i128 != 0 {
                throws.push((self.test_false_target, current_item));
                // println!(
                //     "   Current worry level is not divisible by {}.",
                //     self.test_divisor
                // );
                // println!(
                //     "   Item with worry level {current_item} is thrown to monkey {}.",
                //     self.test_false_target
                // )
            }
        }
        return throws;
    }
}
fn main() {
    let path = Path::new("../../inputs/day11.txt");
    // let path = Path::new("test.txt");
    let raw_text: Vec<String> = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| line.expect("Line read failure"))
        .collect::<Vec<String>>();

    let monkeys: Vec<Monkey> = parse_monkeys(raw_text);

    let part_1_monkey_business = calculate_monkey_business(monkeys.clone(), 20, true);

    println!("Part 1 monkey business = {}", part_1_monkey_business);

    let part_2_monkey_business = calculate_monkey_business(monkeys.clone(), 10000, false);

    println!("Part 2 monkey business = {}", part_2_monkey_business);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_monkeys(text: Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut i = 0;
    for line in text {
        i += 1;
        if line.starts_with("Monkey") {
            monkeys.push(Monkey::new());
            continue;
        }
        let current_monkey = monkeys.len() - 1;
        if line.starts_with("  Starting items: ") {
            monkeys[current_monkey].set_items(
                line.replace("  Starting items: ", "")
                    .split(", ")
                    .map(|x| x.parse().expect("Not a valid item"))
                    .collect::<Vec<i128>>(),
            );
            continue;
        }
        if line == "  Operation: new = old * old" {
            monkeys[current_monkey].set_operator(Operation::Square);
            continue;
        }
        if line.starts_with("  Operation: new = old ") {
            monkeys[current_monkey].set_operator(
                match line
                    .replace("  Operation: new = old ", "")
                    .drain(0..1)
                    .collect::<String>()
                    .as_str()
                {
                    "*" => Operation::Mul,
                    "+" => Operation::Add,
                    _ => panic!("Not valid operator"),
                },
            );
            monkeys[current_monkey].set_operator_value(
                line.replace("  Operation: new = old ", "")
                    .replace("* ", "")
                    .replace("+ ", "")
                    .parse()
                    .expect("Not a value for operator"),
            );
            continue;
        }
        if line.starts_with("  Test: divisible by ") {
            monkeys[current_monkey].set_test_divisor(
                line.replace("  Test: divisible by ", "")
                    .parse()
                    .expect("No value for divisor"),
            );
            continue;
        }
        if line.starts_with("    If true: throw to monkey ") {
            monkeys[current_monkey].set_true_target(
                line.replace("    If true: throw to monkey ", "")
                    .parse()
                    .expect("No value for true target"),
            );
            continue;
        }
        if line.starts_with("    If false: throw to monkey ") {
            monkeys[current_monkey].set_false_target(
                line.replace("    If false: throw to monkey ", "")
                    .parse()
                    .expect("No value for false target"),
            );
            continue;
        }
        if line == "" {
            continue;
        }
        panic!("Line not parsed {i} {line}");
    }
    return monkeys;
}

fn calculate_monkey_business(
    mut monkeys: Vec<Monkey>,
    rounds: i128,
    worry_levels_divide: bool,
) -> i128 {
    let modulo = monkeys
        .iter()
        .map(|x| x.test_divisor)
        .fold(1, |acc, x| acc * x);
    println!("{modulo}");

    for _ in 1..=rounds {
        // println!("Round {x}");
        for current_monkey in 0..monkeys.len() {
            // println!(" Monkey {current_monkey}");
            let throws: Vec<(usize, i128)> = monkeys[current_monkey].update(worry_levels_divide);
            for throw in throws {
                monkeys[throw.0].add_item(throw.1);
            }
        }
        if !worry_levels_divide {
            for current_monkey in 0..monkeys.len() {
                for item in 0..monkeys[current_monkey].items.len() {
                    monkeys[current_monkey].items[item] %= modulo;
                }
            }
        }
        // for current_monkey in 0..monkeys.len() {
        //     print!(
        //         " Monkey {current_monkey} has {} items, ",
        //         monkeys[current_monkey].items.len()
        //     );
        //     for y in 0..monkeys[current_monkey].items.len() {
        //         print!("{}, ", monkeys[current_monkey].items[y])
        //     }
        //     println!()
        // }
        // println!();
        // for a in 0..monkeys.len() {
        //     println!(
        //         "Monkey {a} inspected items {} times.",
        //         monkeys[a].total_items_inspected
        //     );
        // }
        // println!()
    }

    let mut max1 = 0;
    let mut max2 = 0;
    for monkey in monkeys {
        if monkey.total_items_inspected > max1 {
            max2 = max1;
            max1 = monkey.total_items_inspected;
            continue;
        }
        if monkey.total_items_inspected > max2 {
            max2 = monkey.total_items_inspected;
        }
    }
    return max1 * max2;
}
