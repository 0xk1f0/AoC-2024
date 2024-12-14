mod helpers;

use helpers::get_input;
use itertools::{EitherOrBoth::*, Itertools};
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> u64 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // read out lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a vector for statements
    let mut statements: Vec<(u64, Vec<u64>)> = Vec::new();
    // max length
    let mut max_length: u64 = 0;
    // collect chars
    lines.iter().for_each(|line| {
        let line: String = line.chars().collect();
        let result_with_numbers: Vec<&str> = line.split(":").collect();
        let numbers: Vec<u64> = result_with_numbers[1]
            .split(" ")
            .filter(|&particle| particle != "")
            .map(|particle| particle.parse().unwrap())
            .collect();
        max_length = cmp::max(max_length, numbers.len() as u64);
        statements.push((result_with_numbers[0].parse::<u64>().unwrap(), numbers))
    });
    // define operators
    let mut operators: Vec<&str> = Vec::new();
    // use max observed length of numbers vector minus two
    (0..max_length - 2).for_each(|_| {
        operators.extend(["*", "+"].iter());
    });
    // loop over all found statements
    statements
        .iter()
        .map(|statement| {
            // check for all available combinations
            // exit as soon aswe have a valid result using any
            let result = operators
                .iter()
                .combinations(statement.1.len() - 1)
                .unique()
                .any(|combination| {
                    let mut result: u64 = 0;
                    let mut last_operator = "";
                    // perform calculation
                    statement
                        .1
                        .iter()
                        .zip_longest(combination)
                        .for_each(|pair| match pair {
                            Both(l, &r) => {
                                match last_operator {
                                    "+" => {
                                        result += l;
                                    }
                                    "*" => {
                                        result *= l;
                                    }
                                    _ => {
                                        result += l;
                                    }
                                }
                                last_operator = r;
                            }
                            Left(l) => match last_operator {
                                "+" => {
                                    result += l;
                                }
                                "*" => {
                                    result *= l;
                                }
                                _ => {}
                            },
                            Right(_) => {}
                        });
                    // check if result equals statement
                    result == statement.0
                });
            if result {
                statement.0
            } else {
                0
            }
        })
        .sum()
}

fn part_two() -> u64 {
    0
}

fn main() {
    println!("Part 1> {}", part_one());
    println!("Part 2> {}", part_two());
}
