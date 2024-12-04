mod helpers;

use helpers::get_input;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // a regex that matches the multiplication instruction
    // with capture groups for the numbers
    let regex = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    // iterate over the lines of the file
    reader
        .lines()
        .map(|line| {
            let single_line = line.unwrap();
            // find capture groups for every match
            regex
                .captures_iter(&single_line)
                .map(|m| {
                    // multiply second and third capture group
                    // which represent the numbers
                    m[1].parse::<u32>().unwrap() * m[2].parse::<u32>().unwrap()
                })
                .sum::<u32>()
        })
        .sum()
}

fn part_two() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // a regex that matches the multiplication instruction
    // with capture group for accepts and denials
    // and capture groups for the numbers
    let regex = Regex::new(r"don't\(\)|do\(\)|mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    // remember the last instruction
    let mut last_instruction: String = String::from("");
    // iterate over the lines of the file
    reader
        .lines()
        .map(|line| {
            let single_line = line.unwrap();
            // find capture groups for every match
            regex
                .captures_iter(&single_line)
                .map(|m| {
                    if last_instruction != "don't()" && m[0].starts_with("mul") {
                        // if last instr is not dont and current is mul
                        // multiply and return result
                        last_instruction = m[0].to_string();
                        m[1].parse::<u32>().unwrap() * m[2].parse::<u32>().unwrap()
                    } else if last_instruction == "don't()" && m[0].starts_with("mul") {
                        // if last instr was dont and hasnt been cleared by do
                        // dont overwrite last instr and return nothing
                        0
                    } else {
                        // overwrite last instr and return nothing
                        last_instruction = m[0].to_string();
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    println!("Part 1> {}", part_one());
    println!("Part 2> {}", part_two());
}
