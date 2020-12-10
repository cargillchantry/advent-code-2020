use crate::file_util::read_non_blank_lines;
use std::str::FromStr;

#[allow(dead_code)]
pub fn run_day_ten() {
    let mut input = read_non_blank_lines("assets/day_ten")
        .filter_map(|x| usize::from_str(x.as_str()).ok())
        .collect::<Vec<usize>>();
    input.sort_unstable();
    let result = find_jolt_differences(&mut input);
    println!("Result Task 1: {}", result[0] * result[1]);

    let result2 = get_number_of_arrangements(&input);
    println!("Result Task 2: {}", result2);
}

fn get_number_of_arrangements(input: &[usize]) -> usize {
    vec!(0_usize).iter().chain(input)
        .zip(input)
        .fold(
            (0, 0, 1),
            |(x, y, z), (a, b)| match b - a {
                3 => (0, 0, z),
                2 => (z, 0, y + z),
                _ => (y, z, x + y + z)
            }
        ).2
}

fn find_jolt_differences(input: &mut [usize]) -> [usize; 2] {
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
    fn should_produce_number_arrangements() {
        let mut input = [
            28_usize, 33, 18, 42, 31, 14,
            46, 20, 48, 47, 24, 23,
            49, 45, 19, 38, 39, 11,
            1, 32, 25, 35, 8, 17,
            7, 9, 4, 2, 34, 10, 3
        ];
        input.sort_unstable();
        let result = get_number_of_arrangements(&input);
        assert_eq!(result, 19208)
    }

    #[test]
    fn should_produce_jolt_differences() {
        let mut input = [
            28_usize, 33, 18, 42, 31, 14,
            46, 20, 48, 47, 24, 23,
            49, 45, 19, 38, 39, 11,
            1, 32, 25, 35, 8, 17,
            7, 9, 4, 2, 34, 10, 3
        ];
        input.sort_unstable();
        let result = find_jolt_differences(&mut input);
        assert_eq!(result, [22_usize, 10])
    }
}
