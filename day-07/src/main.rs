mod helpers;

use helpers::get_input;
use itertools::{EitherOrBoth::*, Itertools};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> u64 {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // read out lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a vector for statements
    let mut statements: Vec<(u64, Vec<u64>)> = Vec::new();
    // define possible operators
    let operators = ["+", "*"];
    // collect chars
    lines.iter().for_each(|line| {
        let line: String = line.chars().collect();
        let result_with_numbers: Vec<&str> = line.split(":").collect();
        let numbers: Vec<u64> = result_with_numbers[1]
            .split(" ")
            .filter(|&particle| particle != "")
            .map(|particle| particle.parse().unwrap())
            .collect();
        statements.push((result_with_numbers[0].parse::<u64>().unwrap(), numbers))
    });
    // loop over all found statements
    statements
        .iter()
        .map(|statement| {
            // keep track of combinations
            let mut combinations = Vec::new();
            // determine all possible combinations depending on size
            for i in 0..operators.len().pow((statement.1.len() - 1) as u32) {
                let mut sub_combination = Vec::new();
                let mut index = i;
                for _ in 0..statement.1.len() - 1 {
                    let operator = operators[index % operators.len()];
                    sub_combination.push(operator);
                    index /= operators.len();
                }
                combinations.push(sub_combination);
            }
            // check for all available combinations
            // exit as soon as we have a valid result using any
            let result = combinations.iter().any(|combination| {
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
    // open our input file
    let input = File::open(get_input()).unwrap();
    // read out lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a vector for statements
    let mut statements: Vec<(u64, Vec<u64>)> = Vec::new();
    // define possible operators
    let operators = ["+", "*", "||"];
    // collect chars
    lines.iter().for_each(|line| {
        let line: String = line.chars().collect();
        let result_with_numbers: Vec<&str> = line.split(":").collect();
        let numbers: Vec<u64> = result_with_numbers[1]
            .split(" ")
            .filter(|&particle| particle != "")
            .map(|particle| particle.parse().unwrap())
            .collect();
        statements.push((result_with_numbers[0].parse::<u64>().unwrap(), numbers))
    });
    // loop over all found statements
    statements
        .iter()
        .map(|statement| {
            // keep track of combinations
            let mut combinations = Vec::new();
            // determine all possible combinations depending on size
            for i in 0..operators.len().pow((statement.1.len() - 1) as u32) {
                let mut sub_combination = Vec::new();
                let mut index = i;
                for _ in 0..statement.1.len() - 1 {
                    let operator = operators[index % operators.len()];
                    sub_combination.push(operator);
                    index /= operators.len();
                }
                combinations.push(sub_combination);
            }
            // check for all available combinations
            // exit as soon as we have a valid result using any
            let result = combinations.iter().any(|combination| {
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
                                "||" => {
                                    result = format!("{}{}", result, l).parse().unwrap();
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
                            "||" => {
                                result = format!("{}{}", result, l).parse().unwrap();
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

fn main() {
    println!("Part 1> {}", part_one());
    println!("Part 2> {}", part_two());
}
