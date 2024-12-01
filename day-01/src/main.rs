mod helpers;

use helpers::get_input;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // define side vectors
    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();

    // iterate over the lines of the file
    for line in reader.lines() {
        let current_line = line.unwrap();

        // cut out numbers
        let numbers: Vec<&str> = current_line.split("   ").collect();
        let left_number: u32 = numbers.first().unwrap().parse().unwrap();
        let right_number: u32 = numbers.last().unwrap().parse().unwrap();

        // extend the vectors
        left_numbers.push(left_number);
        right_numbers.push(right_number);
    }

    // sort the vectors
    left_numbers.sort();
    right_numbers.sort();

    // calculate the sum
    left_numbers
        .iter()
        .zip(right_numbers.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum()
}

fn part_two() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // define side vectors
    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();

    // iterate over the lines of the file
    for line in reader.lines() {
        let current_line = line.unwrap();

        // cut out numbers
        let numbers: Vec<&str> = current_line.split("   ").collect();
        let left_number: u32 = numbers.first().unwrap().parse().unwrap();
        let right_number: u32 = numbers.last().unwrap().parse().unwrap();

        // extend the vectors
        left_numbers.push(left_number);
        right_numbers.push(right_number);
    }

    // calculate the sum
    left_numbers
        .iter()
        .map(|l| {
            let occurances: u32 = right_numbers.iter().filter(|r| &l == r).map(|_| 1).sum();
            l * occurances
        })
        .sum()
}

fn main() {
    println!("Part 1> {}", part_one());
    println!("Part 2> {}", part_two());
}
