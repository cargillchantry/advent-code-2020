use std::collections::HashMap;

const PUZZLE_INPUT: [usize; 6] = [2, 0, 1, 9, 5, 19];

#[allow(dead_code)]
pub fn run_day_fifteen() {
    let last_spoken = solve_game(&PUZZLE_INPUT, 2020);
    let last_spoken_two = solve_game(&PUZZLE_INPUT, 30000000);
    println!("Task 1 {} and Task 2 {}", last_spoken, last_spoken_two);
}

fn solve_game(input: &[usize], length: usize) -> usize {
    let mut spoken: HashMap<usize, usize> = (&input[0..input.len()])
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i + 1))
        .collect();
    (input.len()+1..=length).fold(
        input.last().copied().unwrap(),
        |last_spoken, current| {
            spoken.insert(last_spoken, current - 1)
                .map(|last| current - last - 1)
                .unwrap_or(0)
        }
    )
}

#[cfg(test)]
mod tests {
    use crate::day_fifteen::*;

    #[test]
    fn should_solve_part_one() {
        assert_eq!(solve_game(&[0,3,6], 2020), 436);
        assert_eq!(solve_game(&[2,1,3], 2020), 10);
        assert_eq!(solve_game(&[1,3,2], 2020), 1);
    }
}
