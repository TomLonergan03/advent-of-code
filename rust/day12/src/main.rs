use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}
fn main() {
    let now = Instant::now();

    let path = Path::new("../../inputs/day12.txt");
    // let path = Path::new("test.txt");
    let raw_text: Vec<String> = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| line.expect("Line read failure"))
        .collect::<Vec<String>>();

    let (map, start, end): (Vec<Vec<i32>>, Location, Location) = parse_map(raw_text);

    let part_1_shortest_distance: i32 = find_shortest_route(map.clone(), start, end);

    println!("Part 1 shortest distance = {}", part_1_shortest_distance);
    println!("In: {:.2?}", now.elapsed());

    let part_2_shortest_possible_distance = find_shortest_possible_route(map.clone(), end);
    println!(
        "Part 2 shortest possible distance = {}",
        part_2_shortest_possible_distance
    );
    println!("In: {:.2?}s", now.elapsed());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_map(text: Vec<String>) -> (Vec<Vec<i32>>, Location, Location) {
    let mut map: Vec<Vec<i32>> = vec![vec![]; text[0].len()];
    let mut start: Location = Location { x: 0, y: 0 };
    let mut end: Location = Location { x: 0, y: 0 };
    let mut row = 0;
    for line in text {
        let mut col = 0;
        for height in line.chars() {
            if height == 'S' {
                start.y = row;
                start.x = col;
                map[col as usize].push(0);
                col += 1;
                continue;
            }
            if height == 'E' {
                end.y = row;
                end.x = col;
                map[col as usize].push(25);
                col += 1;
                continue;
            }
            map[col as usize].push(height as i32 - 97);
            col += 1;
        }
        row += 1;
    }
    return (map, start, end);
}

fn find_shortest_route(map: Vec<Vec<i32>>, start: Location, end: Location) -> i32 {
    let mut distance_to_tile: Vec<Vec<i32>> = vec![vec![-1; map[0].len()]; map.len()];
    distance_to_tile[start.x][start.y] = 0;
    let mut current_distance: i32 = 0;
    let max_distance: i32 = (map.len() * map[0].len()) as i32;
    'outer: loop {
        if current_distance > max_distance {
            println!("\n\nHit max distance with no route\n");
            break;
        }
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                if x == end.x && y == end.y && distance_to_tile[x][y] == current_distance {
                    break 'outer;
                }
                if distance_to_tile[x][y] == current_distance {
                    //look north
                    if y > 0 {
                        if distance_to_tile[x][y - 1] == -1 && map[x][y - 1] - map[x][y] <= 1 {
                            distance_to_tile[x][y - 1] = current_distance + 1;
                        }
                    }
                    //look south
                    if y < map[0].len() - 1 {
                        if distance_to_tile[x][y + 1] == -1 && map[x][y + 1] - map[x][y] <= 1 {
                            distance_to_tile[x][y + 1] = current_distance + 1;
                        }
                    }
                    //look west
                    if x > 0 {
                        if distance_to_tile[x - 1][y] == -1 && map[x - 1][y] - map[x][y] <= 1 {
                            distance_to_tile[x - 1][y] = current_distance + 1;
                        }
                    }
                    //look east
                    if x < map.len() - 1 {
                        if distance_to_tile[x + 1][y] == -1 && map[x + 1][y] - map[x][y] <= 1 {
                            distance_to_tile[x + 1][y] = current_distance + 1;
                        }
                    }
                }
            }
        }
        current_distance += 1;
    }
    return current_distance;
}

fn find_shortest_possible_route(map: Vec<Vec<i32>>, end: Location) -> i32 {
    let mut distance_to_tile: Vec<Vec<i32>> = vec![vec![-1; map[0].len()]; map.len()];
    distance_to_tile[end.x][end.y] = 0;
    let mut current_distance: i32 = 0;
    let max_distance: i32 = (map.len() * map[0].len()) as i32;
    'outer: loop {
        if current_distance > max_distance {
            println!("\n\nHit max distance with no route\n");
            break;
        }
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                if map[x][y] == 0 && distance_to_tile[x][y] == current_distance {
                    break 'outer;
                }
                if distance_to_tile[x][y] == current_distance {
                    //look north
                    if y > 0 {
                        if distance_to_tile[x][y - 1] == -1 && map[x][y - 1] - map[x][y] >= -1 {
                            distance_to_tile[x][y - 1] = current_distance + 1;
                        }
                    }
                    //look south
                    if y < map[0].len() - 1 {
                        if distance_to_tile[x][y + 1] == -1 && map[x][y + 1] - map[x][y] >= -1 {
                            distance_to_tile[x][y + 1] = current_distance + 1;
                        }
                    }
                    //look west
                    if x > 0 {
                        if distance_to_tile[x - 1][y] == -1 && map[x - 1][y] - map[x][y] >= -1 {
                            distance_to_tile[x - 1][y] = current_distance + 1;
                        }
                    }
                    //look east
                    if x < map.len() - 1 {
                        if distance_to_tile[x + 1][y] == -1 && map[x + 1][y] - map[x][y] >= -1 {
                            distance_to_tile[x + 1][y] = current_distance + 1;
                        }
                    }
                }
            }
        }
        current_distance += 1;
    }
    return current_distance;
}

fn _print_tiles(grid: &Vec<Vec<i32>>) {
    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            print!("{:>2} ", grid[x][y])
        }
        println!();
    }
    println!();
}
