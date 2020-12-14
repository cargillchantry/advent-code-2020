use crate::file_util::read_non_blank_lines;
use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
struct Instructions {
    mask: String,
    assignments: Vec<(usize, usize)>
}

#[allow(dead_code)]
pub fn run_day_fourteen() {
    let mut lines = read_non_blank_lines("assets/day_fourteen");
    let parsed = parse_task_one_lines(&mut lines);
    let address_space = execute_task_one(&parsed);
    let sum_part_one: usize = address_space.values().sum();
    println!("Part 1 {}", sum_part_one)
}

fn execute_task_one(instructions: &[Instructions]) -> HashMap<usize, usize> {
    let mut address_space = HashMap::new();
    for instruction in instructions.iter() {
        for assignment in instruction.assignments.iter() {
            let ones_mask = usize::from_str_radix(
                instruction.mask.chars()
                    .map(|x| if x == '1' { '1' } else { '0' })
                    .join("")
                    .as_str(),
                2
            );
            let zeros_mask = usize::from_str_radix(
                instruction.mask.chars()
                    .map(|x| if x == '0' { '0' } else { '1' })
                    .join("")
                    .as_str(),
                2
            );
            if let Some((ones, zeros)) = ones_mask.ok().zip(zeros_mask.ok()) {
                address_space.insert(
                    assignment.0,
                    (assignment.1 | ones) & zeros
                );
            }
        }
    }
    address_space
}

fn parse_task_one_lines(lines: &mut impl Iterator<Item = String>) -> Vec<Instructions> {
    let next_mask = lines.next();
    let mut result = Vec::new();
    if let Some(mut mask) = next_mask {
        let mut assignments = Vec::new();
        while let Some(assignment) = lines.next() {
            let mut split_assignment = assignment.splitn(2, " = ");
            if let Some(split) = split_assignment.next().zip(split_assignment.next()) {
                if split.0 == "mask" {
                    let current_assignments = assignments;
                    assignments = Vec::new();
                    result.push(
                        Instructions {
                            mask, assignments: current_assignments
                        }
                    );
                    mask = assignment;
                } else {
                    let address = usize::from_str(&split.0[4..split.0.len()-1]);
                    let value = usize::from_str(split.1);
                    if let Some(assignment) = address.ok().zip(value.ok()) {
                        assignments.push(assignment);
                    }
                }
            }
        }
        result.push(Instructions { mask, assignments })
    }
    result
}


#[cfg(test)]
mod tests {
    use crate::day_fourteen::*;
}
