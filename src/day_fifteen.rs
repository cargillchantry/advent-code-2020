use std::collections::HashMap;

const PUZZLE_INPUT: [usize; 6] = [2, 0, 1, 9, 5, 19];

#[allow(dead_code)]
pub fn run_day_fifteen() {
    let last_spoken = solve_part_one(&PUZZLE_INPUT, 2020);
    let last_spoken_two = solve_part_one(&PUZZLE_INPUT, 30000000);
    println!("Task 1 {} and Task 2 {}", last_spoken, last_spoken_two);
}

fn solve_part_one(input: &[usize], length: usize) -> usize {
    let mut spoken: HashMap<usize, usize> = input
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i + 1))
        .collect();
    let mut last_spoken = input.last().copied().unwrap();
    spoken.remove(&last_spoken);
    for current in input.len()+1..=length {
        if let Some(x) = spoken.insert(last_spoken, current - 1) {
            last_spoken = current - x - 1;
        } else {
            last_spoken = 0;
        }
    }
    last_spoken
}

#[cfg(test)]
mod tests {
    use crate::day_fifteen::*;

    #[test]
    fn should_solve_part_one() {
        assert_eq!(solve_part_one(&[0,3,6], 2020), 436);
        assert_eq!(solve_part_one(&[2,1,3], 2020), 10);
        assert_eq!(solve_part_one(&[1,3,2], 2020), 1);
    }
}
