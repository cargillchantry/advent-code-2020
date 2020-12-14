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
    let parsed = parse_lines(&mut lines);
    let sum_part_one: usize = execute_task_one(&parsed)
        .values().sum();
    let sum_part_two: usize = execute_task_two(&parsed)
        .values().sum();
    println!("Part 1 {}", sum_part_one);
    println!("Part 2 {}", sum_part_two);
}

fn execute_task_two(instructions: &[Instructions]) -> HashMap<usize, usize> {
    let mut address_space = HashMap::new();
    for instruction in instructions.iter() {
        let ones_mask = get_ones_mask(&instruction.mask);
        let x_masks: Vec<(usize,usize)> = instruction.mask.chars()
            .rev()
            .enumerate()
            .filter(|c| c.1 == 'X')
            .map(|c| 1 << c.0)
            .fold(vec!((0_usize, 0_usize)), |mut buff, elem| {
                let mut to_append = Vec::new();
                for mask in buff.iter_mut() {
                    let mut split = *mask;
                    mask.0 += elem;
                    split.1 += elem;
                    to_append.push(split);
                }
                buff.append(&mut to_append);
                buff
            });

        if let Some(ones_mask_val) = ones_mask {
            for assignment in instruction.assignments.iter() {
                let address = assignment.0 | ones_mask_val;
                for x_mask in x_masks.iter() {
                    address_space.insert((address | x_mask.0) & !x_mask.1, assignment.1);
                }
            }
        }
    }
    address_space
}

fn get_ones_mask(mask: &str) -> Option<usize> {
    usize::from_str_radix(
        mask.chars()
            .map(|x| if x == '1' { '1' } else { '0' })
            .join("")
            .as_str(),
        2
    ).ok()
}

fn execute_task_one(instructions: &[Instructions]) -> HashMap<usize, usize> {
    let mut address_space = HashMap::new();
    for instruction in instructions.iter() {
        for assignment in instruction.assignments.iter() {
            let ones_mask = get_ones_mask(&instruction.mask);
            let zeros_mask = usize::from_str_radix(
                instruction.mask.chars()
                    .map(|x| if x == '0' { '0' } else { '1' })
                    .join("")
                    .as_str(),
                2
            );
            if let Some((ones, zeros)) = ones_mask.zip(zeros_mask.ok()) {
                address_space.insert(
                    assignment.0,
                    (assignment.1 | ones) & zeros
                );
            }
        }
    }
    address_space
}

fn parse_lines(lines: &mut impl Iterator<Item = String>) -> Vec<Instructions> {
    let next_mask = lines.next();
    let mut result = Vec::new();
    if let Some(mut mask) = next_mask {
        let mut assignments = Vec::new();
        for assignment in lines {
            let mut split_assignment = assignment.splitn(2, " = ");
            if let Some(split) = split_assignment.next().zip(split_assignment.next()) {
                if split.0 == "mask" {
                    let current_assignments = assignments;
                    assignments = Vec::new();
                    result.push(
                        Instructions {
                            mask: String::from(&mask[7..]), assignments: current_assignments
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

    #[test]
    fn should_resolve_part_2() {
        let result = execute_task_two(
            &vec!(
                Instructions {
                    mask: String::from("000000000000000000000000000000X1001X"),
                    assignments: vec!((42, 100))
                },
                Instructions {
                    mask: String::from("00000000000000000000000000000000X0XX"),
                    assignments: vec!((26, 1))
                }
            )
        );
        assert_eq!(
            result.values().sum::<usize>(),
            208
        )
    }
}
