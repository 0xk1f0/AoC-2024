mod helpers;

use helpers::get_input;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // read out lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // dependants hashmap
    let mut dependants: HashMap<u32, Vec<u32>> = HashMap::new();
    // collect statements
    let statements: Vec<&String> = lines.iter().filter(|&line| line.contains("|")).collect();
    // collect orderings
    let orderings: Vec<&String> = lines.iter().filter(|&line| line.contains(",")).collect();
    // go through each statement and push a new key, adding all dependants
    statements.iter().for_each(|element| {
        let splits: Vec<u32> = element
            .split('|')
            .map(|el| el.parse::<u32>().unwrap())
            .collect();
        dependants
            .entry(splits[1])
            .or_insert(vec![splits[0]])
            .push(splits[0]);
    });
    // go through all orderings
    orderings
        .iter()
        .map(|element| {
            // filter out numbers
            let splits: Vec<u32> = element
                .split(",")
                .map(|el| el.parse::<u32>().unwrap())
                .collect();
            // keep track of printed
            let mut already_printed: Vec<u32> = Vec::new();
            let is_falsy = splits.iter().any(|&number| {
                let wrong = dependants
                    .get(&number)
                    .unwrap_or(&vec![])
                    .iter()
                    .any(|dependant| {
                        // sequence is incorrect if has not already printed dependant
                        // and is present in current sequence
                        !already_printed.contains(dependant) && splits.contains(dependant)
                    });
                // add printed number
                already_printed.push(number);
                // return exit status
                wrong
            });
            if !is_falsy {
                // calculate median
                splits[(splits.len() - 1) / 2]
            } else {
                0
            }
        })
        .sum()
}

fn part_two() -> u32 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // read out lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // dependants hashmap
    let mut dependants: HashMap<u32, Vec<u32>> = HashMap::new();
    // collect statements
    let statements: Vec<&String> = lines.iter().filter(|&line| line.contains("|")).collect();
    // collect orderings
    let orderings: Vec<&String> = lines.iter().filter(|&line| line.contains(",")).collect();
    // go through each statement and push a new key, adding all dependants
    statements.iter().for_each(|element| {
        let splits: Vec<u32> = element
            .split('|')
            .map(|el| el.parse::<u32>().unwrap())
            .collect();
        dependants
            .entry(splits[1])
            .or_insert(vec![splits[0]])
            .push(splits[0]);
    });
    // go through all orderings
    orderings
        .iter()
        .map(|element| {
            // filter out numbers
            let mut splits: Vec<u32> = element
                .split(",")
                .map(|el| el.parse::<u32>().unwrap())
                .collect();
            // keep track of printed
            let mut already_printed: Vec<u32> = Vec::new();
            // check if we failed at least once
            let mut is_correct_from_beginning = true;
            // keep track of shifted numbers and the current index
            let mut shift_number: u32 = 0;
            let mut at_position: usize = 0;
            // loop until we have corrected all mistakes
            let mut is_correct = false;
            while !is_correct {
                // check correctness for all dependants of number
                is_correct = splits.iter().enumerate().all(|(index, &number)| {
                    let correct = dependants
                        .get(&number)
                        .unwrap_or(&vec![])
                        .iter()
                        .all(|dependant| {
                            // sequence is incorrect if has not already printed dependant
                            // and is present in current sequence
                            if !already_printed.contains(dependant) && splits.contains(dependant) {
                                // remember the number
                                shift_number = dependant.to_owned();
                                // remember the index
                                at_position = index;
                                // void correctness
                                false
                            } else {
                                true
                            }
                        });
                    // add printed number if correct
                    if correct && !already_printed.contains(&number) {
                        already_printed.push(number);
                    }
                    // return exit status
                    correct
                });
                // check if a condition failed
                if !is_correct {
                    // void correctnes from beginning
                    is_correct_from_beginning = false;
                    // find index to delete and remove shifted number
                    splits.remove(splits.iter().position(|&x| x == shift_number).unwrap());
                    // add back shifted number at the correct position
                    splits.insert(at_position, shift_number);
                }
            }
            // check if the sequence is correct from beginning
            // only calc median if not
            if !is_correct_from_beginning {
                splits[(splits.len() - 1) / 2]
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    println!("Part 1> {}", part_one());
    println!("Part 2> {}", part_two());
}
