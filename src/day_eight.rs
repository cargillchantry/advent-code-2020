use crate::file_util::read_non_blank_lines;
use crate::day_eight::Instruction::{NOP, JMP, ACC};
use std::str::FromStr;
use std::collections::HashSet;

enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize)
}

fn parse_isize_from_line(line: &str) -> Option<isize> {
    isize::from_str(&line[4..]).ok()
}

fn get_state_on_loop(instructions: &[Instruction]) -> (usize, isize) {
    let mut visited = HashSet::new();
    let mut current_instruction = 0;
    let mut sum = 0_isize;
    loop {
        if !visited.insert(current_instruction) || current_instruction >= instructions.len() {
            return (current_instruction, sum);
        }
        let instruction = &instructions[current_instruction];
        match instruction {
            ACC(amount) => {
                sum += amount;
            },
            JMP(amount) => {
                if amount.is_negative() {
                    current_instruction -= (*amount).wrapping_abs() as usize;
                } else {
                    current_instruction += *amount as usize;
                }
                continue;
            },
            _ => ()
        }
        current_instruction += 1
    }
}

fn get_bug_free_result(instructions: &mut Vec<Instruction>) -> Option<isize> {
    let mut count = 0;
    while count != instructions.len() {
        let replacement = match instructions[count] {
            NOP(x) => Some(JMP(x)),
            JMP(x) => Some(NOP(x)),
            _ => None
        };
        if let Some(replace) = replacement {
            let current = std::mem::replace(&mut instructions[count], replace);
            let result = get_state_on_loop(&instructions);
            instructions[count] = current;

            if result.0 == instructions.len() {
                return Some(result.1)
            }
        }
        count += 1;
    }
    None
}

#[allow(dead_code)]
pub fn run_day_eight() {
    let mut instructions = read_non_blank_lines("assets/day_eight")
        .filter_map(|line| match &line[..=2] {
            "nop" => Some(NOP(parse_isize_from_line(&line)?)),
            "acc" => Some(ACC(parse_isize_from_line(&line)?)),
            "jmp" => Some(JMP(parse_isize_from_line(&line)?)),
            _ => None
        })
        .collect::<Vec<Instruction>>();
    let result = get_state_on_loop(&instructions);
    let result2 = get_bug_free_result(&mut instructions);

    println!(
        "Loop found at {} with sum {}.",
        result.0,
        result.1
    );
    if let Some(fixed_result) = result2 {
        println!("After fixing we have {}.", fixed_result)
    }
}

#[cfg(test)]
mod tests {
    use crate::day_eight::*;

    #[test]
    fn should_find_the_loop_with_correct_sum() {
        let under_test = vec!(
            NOP(3),
            ACC(3),
            JMP(2),
            ACC(4),
            ACC(5),
            JMP(-2)
        );
        assert_eq!(get_state_on_loop(&under_test), (4, 12))
    }

    #[test]
    fn should_find_the_sum_of_the_correct_program() {
        let mut under_test = vec!(
            NOP(3),
            ACC(3),
            JMP(2),
            ACC(4),
            ACC(5),
            JMP(-2)
        );
        assert_eq!(get_bug_free_result(&mut under_test), Some(8))
    }
}
