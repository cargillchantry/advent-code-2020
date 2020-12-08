use crate::file_util::read_non_blank_lines;
use crate::day_eight::Instruction::{NOP, JMP, ACC};
use std::str::FromStr;
use std::collections::HashSet;

enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize)
}

fn parse_isize_from_line(line: &String) -> Option<isize> {
    isize::from_str(&line[4..]).ok()
}

fn get_loop_info(instructions: &Vec<Instruction>) -> (usize, isize) {
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

fn get_proper_result(instructions: &mut Vec<Instruction>) -> isize {
    let mut sum = 0;
    let mut last_instruction = 0;
    let mut count = 0;
    while last_instruction != instructions.len() && count != instructions.len() {
        let replacement = match instructions[count] {
            NOP(x) => Some(JMP(x)),
            JMP(x) => Some(NOP(x)),
            _ => None
        };
        if let Some(replace) = replacement {
            let current = std::mem::replace(&mut instructions[count], replace);
            let result = get_loop_info(&instructions);
            sum = result.1;
            last_instruction = result.0;
            instructions[count] = current;
        }
        count += 1;
    }
    return sum;
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
    let result = get_loop_info(&instructions);
    let result2 = get_proper_result(&mut instructions);

    println!(
        "Loop found at {} with sum {}. After fixing we have {}.",
        result.0,
        result.1,
        result2
    )
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
        assert_eq!(get_loop_info(&under_test), (4, 12))
    }
}
