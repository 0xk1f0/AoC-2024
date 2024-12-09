mod helpers;

use aho_corasick::AhoCorasick;
use helpers::get_input;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn rotate_matrix(m: &mut Vec<Vec<char>>) {
    *m = (0..m[0].len())
        .map(|oi| (1..m.len() + 1).map(|ii| m[m.len() - ii][oi]).collect())
        .collect();
}

fn part_one() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // with capture groups for the numbers
    let patterns = &["SAMX", "XMAS"];
    let ac = AhoCorasick::new(patterns).unwrap();
    // read out lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define matrix
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    // collect chars
    lines.iter().for_each(|line| {
        let line_chars: Vec<char> = Vec::from_iter(line.chars());
        matrix.push(line_chars);
    });

    // horizontal
    let horizontal: u32 = (0..matrix.len())
        .map(|step| {
            let line: String = matrix[step].iter().collect();
            ac.find_overlapping_iter(&line).count() as u32
        })
        .sum();

    // vertical
    let vertical: u32 = (0..matrix[0].len())
        .map(|step| {
            let line: String = (0..matrix.len())
                .map(|step_inner| matrix[step_inner][step])
                .collect();
            ac.find_overlapping_iter(&line).count() as u32
        })
        .sum();

    // diagonal_left_down iteration 1
    let diagonal_1: u32 = (0..matrix.len())
        .map(|step| {
            let line: String = (0..matrix[0].len() - step)
                .map(|step_inner| matrix[step + step_inner][step_inner])
                .collect();
            ac.find_overlapping_iter(&line).count() as u32
        })
        .sum();

    // rotate matrix clockwise
    rotate_matrix(&mut matrix);

    // diagonal_left_down iteration 2
    let diagonal_2: u32 = (0..matrix.len())
        .map(|step| {
            let line: String = (0..matrix[0].len() - step)
                .map(|step_inner| matrix[step + step_inner][step_inner])
                .collect();
            ac.find_overlapping_iter(&line).count() as u32
        })
        .sum();

    // rotate matrix clockwise
    rotate_matrix(&mut matrix);

    // diagonal_left_down iteration 3, dont include center
    let diagonal_3: u32 = (1..matrix.len())
        .map(|step| {
            let line: String = (0..matrix[0].len() - step)
                .map(|step_inner| matrix[step + step_inner][step_inner])
                .collect();
            ac.find_overlapping_iter(&line).count() as u32
        })
        .sum();

    // rotate matrix clockwise
    rotate_matrix(&mut matrix);

    // diagonal_left_down iteration 4, dont include center
    let diagonal_4: u32 = (1..matrix.len())
        .map(|step| {
            let line: String = (0..matrix[0].len() - step)
                .map(|step_inner| matrix[step + step_inner][step_inner])
                .collect();
            ac.find_overlapping_iter(&line).count() as u32
        })
        .sum();

    // add up sums
    horizontal + vertical + diagonal_1 + diagonal_2 + diagonal_3 + diagonal_4
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
    // iterate entire matrix
    (0..matrix.len())
        .map(|y| {
            (0..matrix[0].len())
                .map(|x| {
                    // check for A center case
                    if matrix[y][x] == 'A' {
                        // check if we ever travel out of bounds
                        if x > 0 && y > 0 && x < matrix[0].len() - 1 && y < matrix.len() - 1 {
                            // check subcase 1
                            if matrix[y - 1][x - 1] == 'S'
                                && matrix[y + 1][x + 1] == 'M'
                                && matrix[y + 1][x - 1] == 'S'
                                && matrix[y - 1][x + 1] == 'M'
                            {
                                return 1;
                            }
                            // check subcase 2
                            else if matrix[y - 1][x - 1] == 'M'
                                && matrix[y + 1][x + 1] == 'S'
                                && matrix[y + 1][x - 1] == 'M'
                                && matrix[y - 1][x + 1] == 'S'
                            {
                                return 1;
                            }
                            // check subcase 3
                            else if matrix[y - 1][x - 1] == 'M'
                                && matrix[y + 1][x + 1] == 'S'
                                && matrix[y + 1][x - 1] == 'S'
                                && matrix[y - 1][x + 1] == 'M'
                            {
                                return 1;
                            }
                            // check subcase 4
                            else if matrix[y - 1][x - 1] == 'S'
                                && matrix[y + 1][x + 1] == 'M'
                                && matrix[y + 1][x - 1] == 'M'
                                && matrix[y - 1][x + 1] == 'S'
                            {
                                return 1;
                            }
                        }
                    }
                    // else dont count
                    0
                })
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    println!("Part 1> {}", part_one());
    println!("Part 2> {}", part_two());
}
