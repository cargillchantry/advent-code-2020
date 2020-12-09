use crate::file_util::read_non_blank_lines;
use std::str::FromStr;

#[allow(dead_code)]
pub fn run_day_nine() {
    let lines = read_non_blank_lines("assets/day_nine")
        .filter_map(|line| isize::from_str(line.as_str()).ok())
        .collect::<Vec<isize>>();

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

fn solve_part_two(number: isize, numbers: &[isize]) -> Option<(isize, isize)> {
    let mut sum;
    let mut smallest;
    let mut largest;
    for i in 0..numbers.len() {
        sum = numbers[i];
        smallest = sum;
        largest = sum;
        for &value in numbers.iter().skip(i + 1) {
            sum += value;
            if value < smallest {
                smallest = value;
            }
            if value > largest {
                largest = value;
            }
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

fn solve_part_one(data: &[isize]) -> Option<isize> {
    let mut buffer = [0; 25];
    let mut manipulated_buffer = [0; 25];
    let mut iter = data.iter();
    read_into_buffers(&mut buffer, &mut manipulated_buffer, &mut iter);

    iter
        .enumerate()
        .find(|(index, &x)| {
            let is_result = !is_number_sum_of_any(x, &mut manipulated_buffer);
            let replacing = buffer[index % 25];
            buffer[index % 25] = x;
            if let Some(position) = manipulated_buffer.iter().position(|&it| it == replacing) {
                manipulated_buffer[position] = x;
            }
            is_result
        })
        .map(|result| *result.1)
}

fn read_into_buffers<'a>(
    buffer: &mut [isize; 25],
    other_buffer: &mut [isize; 25],
    iter: &mut impl Iterator<Item = &'a isize>
) {
    iter.take(25).enumerate().for_each(|(index, n)| {
        buffer[index] = *n;
        other_buffer[index] = *n;
    })
}

fn is_number_sum_of_any(value: isize, numbers: &mut[isize]) -> bool {
    numbers.sort_unstable();
    'outer: for x in 0..numbers.len() {
        if numbers[x] > value {
            break;
        }
        for &y in numbers.iter().skip(x + 1) {
            if numbers[x] + y == value {
                return true
            }
            if numbers[x] + y > value {
                continue 'outer
            }
        }
    }
    false
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
