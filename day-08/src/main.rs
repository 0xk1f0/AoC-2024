mod helpers;

use helpers::get_input;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // read out lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define matrix
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    // antennas hashmap
    let mut antennas: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    // collect chars
    lines.iter().for_each(|line| {
        let line_chars: Vec<char> = Vec::from_iter(line.chars());
        matrix.push(line_chars);
    });
    // collect antenna definitions and positions
    matrix.iter().enumerate().for_each(|(yy, rows)| {
        rows.iter().enumerate().for_each(|(xx, &tuned_frequency)| {
            if tuned_frequency != '.' {
                antennas
                    .entry(tuned_frequency)
                    .or_insert(HashSet::from([(xx, yy)]))
                    .insert((xx, yy));
            }
        })
    });
    // go through each frequency sub nodes
    antennas.iter().for_each(|(_, nodes)| {
        nodes.iter().for_each(|&sub_node| {
            nodes.iter().for_each(|&sub_neighbor| {
                let mut near_multipliers: (i32, i32) = (1, 1);
                let mut far_multipliers: (i32, i32) = (1, 1);
                // check for neighbor location
                if sub_neighbor == sub_node {
                    // if we met ourselves again, say goodbye
                    return;
                }
                if sub_neighbor.0 > sub_node.0 {
                    far_multipliers.0 = 2;
                    near_multipliers.0 = -1;
                }
                if sub_neighbor.1 > sub_node.1 {
                    far_multipliers.1 = 2;
                    near_multipliers.1 = -1;
                }
                if sub_neighbor.0 < sub_node.0 {
                    far_multipliers.0 = -2;
                    near_multipliers.0 = 1;
                }
                if sub_neighbor.1 < sub_node.1 {
                    far_multipliers.1 = -2;
                    near_multipliers.1 = 1;
                }
                // get absolute difference between the two nodes
                let absolute_diff: (usize, usize) = (
                    sub_neighbor.0.abs_diff(sub_node.0),
                    sub_neighbor.1.abs_diff(sub_node.1),
                );
                // perform out of bounds check for nearest antinode
                if sub_node.0 as i32 + (absolute_diff.0 as i32 * near_multipliers.0) > -1
                    && sub_node.1 as i32 + (absolute_diff.1 as i32 * near_multipliers.1) > -1
                    && sub_node.0 as i32 + (absolute_diff.0 as i32 * near_multipliers.0)
                        < matrix[0].len() as i32
                    && sub_node.1 as i32 + (absolute_diff.1 as i32 * near_multipliers.1)
                        < matrix.len() as i32
                {
                    // compute new locations for nearest antinode if we're in bounds
                    let node_x = sub_node.0 as i32 + (absolute_diff.0 as i32 * near_multipliers.0);
                    let node_y = sub_node.1 as i32 + (absolute_diff.1 as i32 * near_multipliers.1);
                    // replace as antinode in matrix
                    matrix[node_y as usize][node_x as usize] = '#';
                }
                // perform out of bounds check for furthest antinode
                if sub_node.0 as i32 + (absolute_diff.0 as i32 * far_multipliers.0)
                    < matrix[0].len() as i32
                    && sub_node.1 as i32 + (absolute_diff.1 as i32 * far_multipliers.1)
                        < matrix.len() as i32
                    && sub_node.0 as i32 + (absolute_diff.0 as i32 * far_multipliers.0) > -1
                    && sub_node.1 as i32 + (absolute_diff.1 as i32 * far_multipliers.1) > -1
                {
                    // compute new locations for furthest antinode if we're in bounds
                    let node_x = sub_node.0 as i32 + (absolute_diff.0 as i32 * far_multipliers.0);
                    let node_y = sub_node.1 as i32 + (absolute_diff.1 as i32 * far_multipliers.1);
                    // replace as antinode in matrix
                    matrix[node_y as usize][node_x as usize] = '#';
                }
            });
        });
    });
    // find all antinodes and count/sum them up
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|char| if *char == '#' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn part_two() -> usize {
    // open our input file
    let input = File::open(get_input()).unwrap();
    // read out lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define matrix
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    // antennas hashmap
    let mut antennas: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    // collect chars
    lines.iter().for_each(|line| {
        let line_chars: Vec<char> = Vec::from_iter(line.chars());
        matrix.push(line_chars);
    });
    // collect antenna definitions and positions
    matrix.iter().enumerate().for_each(|(yy, rows)| {
        rows.iter().enumerate().for_each(|(xx, &tuned_frequency)| {
            if tuned_frequency != '.' {
                antennas
                    .entry(tuned_frequency)
                    .or_insert(HashSet::from([(xx, yy)]))
                    .insert((xx, yy));
            }
        })
    });
    // go through each frequency sub nodes
    antennas.iter().for_each(|(_, nodes)| {
        nodes.iter().for_each(|&sub_node| {
            nodes.iter().for_each(|&sub_neighbor| {
                let mut near_multipliers: (i32, i32) = (1, 1);
                let mut far_multipliers: (i32, i32) = (1, 1);
                // check for neighbor location
                if sub_neighbor == sub_node {
                    // if we met ourselves again, say goodbye
                    return;
                }
                if sub_neighbor.0 > sub_node.0 {
                    far_multipliers.0 = 1;
                    near_multipliers.0 = -1;
                }
                if sub_neighbor.1 > sub_node.1 {
                    far_multipliers.1 = 1;
                    near_multipliers.1 = -1;
                }
                if sub_neighbor.0 < sub_node.0 {
                    far_multipliers.0 = -1;
                    near_multipliers.0 = 1;
                }
                if sub_neighbor.1 < sub_node.1 {
                    far_multipliers.1 = -1;
                    near_multipliers.1 = 1;
                }
                // get absolute difference between the two nodes
                let absolute_diff: (usize, usize) = (
                    sub_neighbor.0.abs_diff(sub_node.0),
                    sub_neighbor.1.abs_diff(sub_node.1),
                );
                // keep track of nearest antinode iteration
                let mut stepper_nearest: i32 = 1;
                // perform out of bounds check for each nearest antinode
                while sub_node.0 as i32
                    + ((absolute_diff.0 as i32 * near_multipliers.0) * stepper_nearest)
                    > -1
                    && sub_node.1 as i32
                        + ((absolute_diff.1 as i32 * near_multipliers.1) * stepper_nearest)
                        > -1
                    && sub_node.0 as i32
                        + ((absolute_diff.0 as i32 * near_multipliers.0) * stepper_nearest)
                        < matrix[0].len() as i32
                    && sub_node.1 as i32
                        + ((absolute_diff.1 as i32 * near_multipliers.1) * stepper_nearest)
                        < matrix.len() as i32
                {
                    // compute new locations for nearest antinode if we're in bounds
                    let node_x = sub_node.0 as i32
                        + ((absolute_diff.0 as i32 * near_multipliers.0) * stepper_nearest);
                    let node_y = sub_node.1 as i32
                        + ((absolute_diff.1 as i32 * near_multipliers.1) * stepper_nearest);
                    // replace as antinode in matrix
                    matrix[node_y as usize][node_x as usize] = '#';
                    // step by one
                    stepper_nearest += 1;
                }
                // keep track of furthest antinode iteration
                let mut stepper_furthest: i32 = 1;
                // perform out of bounds check for each furthest antinode
                while sub_node.0 as i32
                    + ((absolute_diff.0 as i32 * far_multipliers.0) * stepper_furthest)
                    < matrix[0].len() as i32
                    && sub_node.1 as i32
                        + ((absolute_diff.1 as i32 * far_multipliers.1) * stepper_furthest)
                        < matrix.len() as i32
                    && sub_node.0 as i32
                        + ((absolute_diff.0 as i32 * far_multipliers.0) * stepper_furthest)
                        > -1
                    && sub_node.1 as i32
                        + ((absolute_diff.1 as i32 * far_multipliers.1) * stepper_furthest)
                        > -1
                {
                    // compute new locations for furthest antinode if we're in bounds
                    let node_x = sub_node.0 as i32
                        + ((absolute_diff.0 as i32 * far_multipliers.0) * stepper_furthest);
                    let node_y = sub_node.1 as i32
                        + ((absolute_diff.1 as i32 * far_multipliers.1) * stepper_furthest);
                    // replace as antinode in matrix
                    matrix[node_y as usize][node_x as usize] = '#';
                    // step by one
                    stepper_furthest += 1;
                }
            });
        });
    });
    // find all antinodes and count/sum them up
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|char| if *char == '#' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    println!("Part 1> {}", part_one());
    println!("Part 2> {}", part_two());
}
