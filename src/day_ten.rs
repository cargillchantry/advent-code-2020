use crate::file_util::read_non_blank_lines;
use std::str::FromStr;

#[allow(dead_code)]
pub fn run_day_ten() {
    let task_one = read_non_blank_lines("assets/day_ten")
        .filter_map(|x| usize::from_str(x.as_str()).ok());
    let result = find_jolt_differences(&mut task_one.collect::<Vec<usize>>());
    println!("Result Task 1: {}", result[0] * result[1])
}

fn find_jolt_differences(input: &mut [usize]) -> [usize; 2] {
    input.sort_unstable();
    let mut result = [1, 1];
    input.iter().zip(&input[1..])
        .map(|(x, y)| y - x)
        .for_each(|diff| {
            if diff == 1 {
                result[0] += 1;
            } else if diff == 3 {
                result[1] += 1;
            }
        });
    result
}

#[cfg(test)]
mod tests {
    use crate::day_ten::*;

    #[test]
    fn should_produce_jolt_differences() {
        let mut input = [
            28_usize, 33, 18, 42, 31, 14,
            46, 20, 48, 47, 24, 23,
            49, 45, 19, 38, 39, 11,
            1, 32, 25, 35, 8, 17,
            7, 9, 4, 2, 34, 10, 3
        ];
        let result = find_jolt_differences(&mut input);
        assert_eq!(result, [22_usize, 10])
    }
}
