mod helpers;

use helpers::get_input;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // safe numbers
    let mut safe_reports: u32 = 0;
    // iterate over the lines of the file
    reader.lines().for_each(|line| {
        let current_line = line.unwrap();
        // cut out level_numbers
        let level_numbers: Vec<&str> = current_line.split(" ").collect();
        // remember last number
        let mut last_number: u32 = 0;
        // check if sequence is increasing
        let mut increasing: bool = false;
        // violation marker for illegal events
        let mut violation: bool = false;
        // check if level_numbers are safe
        level_numbers.iter().enumerate().for_each(|(turn, &num)| {
            let this_number: u32 = num.parse().unwrap();
            // skip for first turn
            if turn == 0 {
                last_number = this_number;
            } else {
                // check if we're increasing or decreasing
                let last_increasing = increasing;
                if this_number > last_number {
                    increasing = true;
                } else {
                    increasing = false;
                }
                // if it's past our second turn, check if direction has changed
                if turn != 1 && last_increasing != increasing {
                    violation = true;
                    return;
                }
                // check if diff is in allowed bounds
                let diff = last_number.abs_diff(this_number);
                if diff < 1 || diff > 3 {
                    violation = true;
                    return;
                }
                last_number = this_number;
            }
        });
        // check for violation, else add to sum
        if !violation {
            safe_reports += 1;
        }
    });
    safe_reports
}

fn part_two() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // safe numbers
    let mut safe_reports: u32 = 0;
    // iterate over the lines of the file
    reader.lines().for_each(|line| {
        let current_line = line.unwrap();
        // cut out level_numbers
        let level_numbers: Vec<&str> = current_line.split(" ").collect();
        // check for a pass
        let mut passed: bool = false;
        // repeat process until we get a pass or run out of pssible removals
        (0..level_numbers.len() + 1).for_each(|retry| {
            let mut modded_numbers = level_numbers.clone();
            // if not initial, remove a number with index of current retry minus one
            if retry != 0 {
                modded_numbers.remove(retry - 1);
            }
            // remember last number
            let mut last_number: u32 = 0;
            // check if sequence is increasing
            let mut increasing: bool = false;
            // violation marker for illegal events
            let mut violation: bool = false;
            // check if level_numbers are safe
            modded_numbers.iter().enumerate().for_each(|(turn, &num)| {
                let this_number: u32 = num.parse().unwrap();
                // skip for first turn
                if turn == 0 {
                    last_number = this_number;
                } else {
                    // check if we're increasing or decreasing
                    let last_increasing = increasing;
                    if this_number > last_number {
                        increasing = true;
                    } else {
                        increasing = false;
                    }
                    // if it's past our second turn, check if direction has changed
                    if turn != 1 && last_increasing != increasing {
                        violation = true;
                        return;
                    }
                    // check if diff is in allowed bounds
                    let diff = last_number.abs_diff(this_number);
                    if diff < 1 || diff > 3 {
                        violation = true;
                        return;
                    }
                    last_number = this_number;
                }
            });
            // check for violation and if already passed, else add to sum
            if !violation && !passed {
                passed = true;
                safe_reports += 1;
            }
        });
    });
    safe_reports
}

fn main() {
    println!("Part 1> {}", part_one());
    println!("Part 2> {}", part_two());
}
