mod helpers;

use helpers::get_input;
use std::fs::File;
use std::io::{BufRead, BufReader};

// to keep track of direction
#[derive(Debug)]
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

    0
}

fn main() {
    println!("Part 1> {}", part_one());
    println!("Part 2> {}", part_two());
}
