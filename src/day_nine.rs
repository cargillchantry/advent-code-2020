use crate::file_util::read_non_blank_lines;
use std::str::FromStr;
use std::cmp::{min, max};

#[allow(dead_code)]
pub fn run_day_nine() {
    let lines = read_non_blank_lines("assets/day_nine")
        .filter_map(|line| usize::from_str(line.as_str()).ok())
        .collect::<Vec<usize>>();

    let calculated_result = solve_part_one(&lines);

    if let Some(result) = calculated_result {
        println!("Result P1: {}", result);
        let result2 = solve_part_two(result, &lines);
        if let Some((smallest, largest)) = result2 {
            println!(
               "Result P2: Smallest {} and Largest {} sum to {}",
               smallest,
               largest,
               smallest + largest
            )
        }
    }
}

fn solve_part_one(data: &[usize]) -> Option<usize> {
    let mut history = [0; 25];
    let mut manipulated_history = [0; 25];
    let mut iter = data.iter();
    read_into_buffers(&mut history, &mut manipulated_history, &mut iter);

    iter
        .enumerate()
        .find(|(index, &x)| {
            let is_result = !is_number_sum_of_any(x, &mut manipulated_history);
            let replacing = history[index % 25];
            history[index % 25] = x;
            if let Some(position) = manipulated_history.iter().position(|&it| it == replacing) {
                manipulated_history[position] = x;
            }
            is_result
        })
        .map(|result| *result.1)
}

fn read_into_buffers<'a>(
    buffer: &mut [usize; 25],
    other_buffer: &mut [usize; 25],
    iter: &mut impl Iterator<Item = &'a usize>
) {
    iter.take(25).enumerate().for_each(|(index, n)| {
        buffer[index] = *n;
        other_buffer[index] = *n;
    })
}

fn is_number_sum_of_any(value: usize, numbers: &mut[usize]) -> bool {
    numbers.sort_unstable();
    for x in 0..numbers.len() {
        let current_value = numbers[x];
        let other = numbers[x+1..]
            .binary_search_by(|other| (other + current_value).cmp(&value));
        if other.is_ok() {
            return true;
        }
    }
    false
}

fn solve_part_two(number: usize, numbers: &[usize]) -> Option<(usize, usize)> {
    let mut sum;
    let mut smallest;
    let mut largest;
    for i in 0..numbers.len() {
        sum = numbers[i];
        smallest = sum;
        largest = sum;
        for &value in numbers.iter().skip(i + 1) {
            sum += value;
            smallest = min(smallest, value);
            largest = max(largest, value);
            if sum == number {
                return Some((smallest, largest));
            }
            if sum > number {
                break;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::day_nine::*;

    #[test]
    fn should_find_bounds_of_continuous_sum_matching_number() {
        assert_eq!(
            solve_part_two(
                17,
                &vec!(1, 32, 4, 7, 6, 78)
            ),
            Some((4, 7))
        )
    }

    #[test]
    fn should_determine_if_number_is_sum_of_others() {
        assert_eq!(
            is_number_sum_of_any(
                25,
                &mut [1, 24]
            ),
            true
        );

        assert_eq!(
            is_number_sum_of_any(
                25,
                &mut [1, 23]
            ),
            false
        );

        assert_eq!(
            is_number_sum_of_any(
                25,
                &mut [1, 7, 8, 22, 3, 13, 24]
            ),
            true
        );
    }
}
