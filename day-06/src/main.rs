mod helpers;

use helpers::get_input;
use std::fs::File;
use std::io::{BufRead, BufReader};

// to keep track of direction
#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// implement a turn directive
impl Direction {
    fn turn(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn part_one() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // read out lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define matrix
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    // collect chars
    lines.iter().for_each(|line| {
        let line_chars: Vec<char> = Vec::from_iter(line.chars());
        matrix.push(line_chars);
    });
    // keep track of virtual steps
    let mut steps_taken: u32 = 0;
    // keep track of visited position
    let mut visited: Vec<(i32, i32)> = Vec::new();
    // define cursor position
    let mut cursor_x: i32 = 0;
    let mut cursor_y: i32 = 0;
    // find initial position represented by '^'
    matrix.iter().enumerate().any(|(y, sub_vector)| {
        let pos = sub_vector.iter().position(|&initial_x| initial_x == '^');
        if pos.is_some() {
            // adjust our starting cursor
            cursor_x = pos.unwrap() as i32;
            cursor_y = y as i32;
            // add a new visited postion
            visited.push((cursor_x, cursor_y));
            // and step
            steps_taken += 1;
            true
        } else {
            false
        }
    });
    // define next cursor values
    let mut cursor_next_x: i32 = cursor_x;
    let mut cursor_next_y: i32 = cursor_y;
    // define inital direction
    let mut direction: Direction = Direction::Up;
    // loop as long as we are not out of bounds
    while cursor_x < (matrix[0].len() - 1) as i32
        && cursor_y < (matrix.len() - 1) as i32
        && cursor_x > 0
        && cursor_y > 0
    {
        // check direction
        // and fulfill move depending on it
        // if we hit an obstacle, turn
        match direction {
            Direction::Up => {
                if matrix[(cursor_y - 1) as usize][cursor_x as usize] == '#' {
                    direction = direction.turn();
                } else {
                    cursor_next_y = cursor_y - 1;
                }
            }
            Direction::Down => {
                if matrix[(cursor_y + 1) as usize][cursor_x as usize] == '#' {
                    direction = direction.turn();
                } else {
                    cursor_next_y = cursor_y + 1;
                }
            }
            Direction::Left => {
                if matrix[cursor_y as usize][(cursor_x - 1) as usize] == '#' {
                    direction = direction.turn();
                } else {
                    cursor_next_x = cursor_x - 1;
                }
            }
            Direction::Right => {
                if matrix[cursor_y as usize][(cursor_x + 1) as usize] == '#' {
                    direction = direction.turn();
                } else {
                    cursor_next_x = cursor_x + 1;
                }
            }
        }
        // check if our cursor changed
        if cursor_x != cursor_next_x || cursor_y != cursor_next_y {
            // apply next cursor
            cursor_x = cursor_next_x;
            cursor_y = cursor_next_y;
            // check if we already visited here
            if !visited.contains(&(cursor_x, cursor_y)) {
                steps_taken += 1;
                visited.push((cursor_x, cursor_y));
            }
        }
    }
    // return sum
    steps_taken
}

fn part_two() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // read out lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define matrix
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    // collect chars
    lines.iter().for_each(|line| {
        let line_chars: Vec<char> = Vec::from_iter(line.chars());
        matrix.push(line_chars);
    });
    // keep track of possible blockages
    let mut possibilites: u32 = 0;
    // iterate through every possible position in matrix
    matrix.iter().enumerate().for_each(|(yy, rows)| {
        rows.iter().enumerate().for_each(|(xx, _)| {
            // clone the matrix to a temp matrix
            let mut temp_matrix = matrix.clone();
            // set a blockage for current index
            temp_matrix[yy][xx] = '#';
            // keep track of visited position
            let mut visited: Vec<(i32, i32, Direction)> = Vec::new();
            // define inital direction
            let mut direction: Direction = Direction::Up;
            // define cursor position
            let mut cursor_x: i32 = 0;
            let mut cursor_y: i32 = 0;
            // find initial position represented by '^'
            temp_matrix.iter().enumerate().any(|(y, sub_vector)| {
                let pos = sub_vector.iter().position(|&initial_x| initial_x == '^');
                if pos.is_some() {
                    // adjust our starting cursor
                    cursor_x = pos.unwrap() as i32;
                    cursor_y = y as i32;
                    // add a new visited postion
                    visited.push((cursor_x, cursor_y, direction));
                    true
                } else {
                    false
                }
            });
            // define next cursor values
            let mut cursor_next_x: i32 = cursor_x;
            let mut cursor_next_y: i32 = cursor_y;
            // stop looping if we start to repeat
            let mut handbrake = false;
            // loop as long as we are not out of bounds
            while cursor_x < (temp_matrix[0].len() - 1) as i32
                && cursor_y < (temp_matrix.len() - 1) as i32
                && cursor_x > 0
                && cursor_y > 0
                && handbrake == false
            {
                // check direction
                // and fulfill move depending on it
                // if we hit an obstacle, turn
                match direction {
                    Direction::Up => {
                        if temp_matrix[(cursor_y - 1) as usize][cursor_x as usize] == '#' {
                            direction = direction.turn();
                        } else {
                            cursor_next_y = cursor_y - 1;
                        }
                    }
                    Direction::Down => {
                        if temp_matrix[(cursor_y + 1) as usize][cursor_x as usize] == '#' {
                            direction = direction.turn();
                        } else {
                            cursor_next_y = cursor_y + 1;
                        }
                    }
                    Direction::Left => {
                        if temp_matrix[cursor_y as usize][(cursor_x - 1) as usize] == '#' {
                            direction = direction.turn();
                        } else {
                            cursor_next_x = cursor_x - 1;
                        }
                    }
                    Direction::Right => {
                        if temp_matrix[cursor_y as usize][(cursor_x + 1) as usize] == '#' {
                            direction = direction.turn();
                        } else {
                            cursor_next_x = cursor_x + 1;
                        }
                    }
                }
                // check if our cursor changed
                if cursor_x != cursor_next_x || cursor_y != cursor_next_y {
                    // apply next cursor
                    cursor_x = cursor_next_x;
                    cursor_y = cursor_next_y;
                    // check if we already visited here and the direction
                    // is equal to the last time we visited
                    // which means we are looping
                    if visited.contains(&(cursor_x, cursor_y, direction)) {
                        // add one to possible loops and pull handbrake
                        possibilites += 1;
                        handbrake = true;
                    } else {
                        visited.push((cursor_x, cursor_y, direction));
                    }
                }
            }
        })
    });
    // return number of possible blockages
    possibilites
}

fn main() {
    println!("Part 1> {}", part_one());
    println!("Part 2> {}", part_two());
}
