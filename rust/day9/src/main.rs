use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Down => write!(f, "Down"),
            Direction::Up => write!(f, "Up"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

fn main() {
    let path = Path::new("../../inputs/day9.txt");
    // let path = Path::new("test.txt");
    let raw_text: Vec<String> = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| Result::expect(line, "Line read failure"))
        .collect::<Vec<String>>();

    let moves: Vec<Direction> = get_moves(raw_text);

    let part_1_visited_tiles = total_visited_squares(&moves, 2);

    let part_2_visited_tiles = total_visited_squares(&moves, 10);

    println!("Part 1 number of tiles visited = {}", part_1_visited_tiles);
    println!("Part 2 number of tiles visited = {}", part_2_visited_tiles);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_moves(text: Vec<String>) -> Vec<Direction> {
    let mut moves: Vec<Direction> = vec![];
    for mut line in text {
        let move_direction = (match line.drain(0..1).collect::<String>().as_str() {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Not a direction"),
        })
        .expect("Not a direction");
        let move_number: i32 = line.replace(" ", "").parse().expect("Invalid number");

        for _ in 0..move_number {
            moves.push(move_direction.clone());
        }
    }
    return moves;
}

fn total_visited_squares(moves: &Vec<Direction>, length: i32) -> i32 {
    let grid_size = 1000;
    let start_point = grid_size / 2;
    let mut grid: Vec<Vec<bool>> = vec![vec![false; grid_size]; grid_size];
    let head: Location = Location {
        x: (start_point as i32),
        y: (start_point as i32),
    };
    let mut rope: Vec<Location> = vec![head];
    for _ in 0..(length - 1) {
        rope.push(head.clone())
    }

    grid[start_point][start_point] = true;

    for single_move in moves {
        // update head
        match single_move {
            Direction::Up => rope[0].y += 1,
            Direction::Down => rope[0].y -= 1,
            Direction::Left => rope[0].x -= 1,
            Direction::Right => rope[0].x += 1,
        }

        // update tail
        for x in 1..rope.len() {
            rope[x] = update_knot(rope[x - 1], rope[x]);
        }
        grid[rope[rope.len() - 1].x as usize][rope[rope.len() - 1].y as usize] = true;
    }

    let mut total = 0;
    for col in grid {
        for row in col {
            if row {
                total += 1;
            }
        }
    }

    return total;
}

fn update_knot(leader: Location, mut follower: Location) -> Location {
    if ((leader.x as f32 - follower.x as f32).powf(2.0)
        + (leader.y as f32 - follower.y as f32).powf(2.0))
    .sqrt()
        <= 2.0_f32.sqrt()
    {
        return follower;
    }

    let x_offset: i32 = (leader.x - follower.x) as i32;
    let y_offset: i32 = (leader.y - follower.y) as i32;

    if (x_offset.abs() > 1 && y_offset.abs() > 0) || (x_offset.abs() > 0 && y_offset.abs() > 1) {
        follower.x += x_offset / x_offset.abs();
        follower.y += y_offset / y_offset.abs();
        return follower;
    }
    if x_offset.abs() > 1 && y_offset == 0 {
        follower.x += x_offset / x_offset.abs();
        return follower;
    }
    if x_offset == 0 && y_offset.abs() > 1 {
        follower.y += y_offset / y_offset.abs();
        return follower;
    }
    panic!(
        "Not possible to resolve follower, current leader ({},{}), follower ({},{})",
        leader.x, leader.y, follower.x, follower.y
    );
}

fn _render_grid(grid: &mut Vec<Vec<bool>>, rope: &Vec<Location>) {
    println!("Head at ({},{})", rope[0].x, rope[0].y);
    for y in (0..grid.len()).rev() {
        for x in 0..grid.len() {
            if rope.contains(&Location {
                x: x as i32,
                y: y as i32,
            }) {
                print!(
                    "{}",
                    rope.iter()
                        .position(|&r| r
                            == Location {
                                x: x as i32,
                                y: y as i32,
                            })
                        .expect("Location doesn't exist")
                );
            } else {
                print!(".")
            }
        }
        println!()
    }
}
