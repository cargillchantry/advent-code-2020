use crate::file_util::read_lines;
use std::num::ParseIntError;
use itertools::Itertools;

fn to_integer_from_binary_string(binary_str: &str, ones_char: char) -> Result<u32, ParseIntError> {
    let binary_representation: String = binary_str.chars()
        .map(|curr| if curr == ones_char { '1' } else { '0' })
        .collect();
    u32::from_str_radix(binary_representation.as_str(), 2)
}

#[allow(dead_code)]
pub fn run_day_five() {
    let converted: Vec<u32> = read_lines("assets/day_five")
        .filter_map(|line| {
            let row = to_integer_from_binary_string(&line[..=6], 'B').ok()?;
            let seat = to_integer_from_binary_string(&line[7..=9], 'R').ok()?;
            Some(8 * row + seat)
        })
        .sorted()
        .collect();
    let task_two = converted.windows(2).find(|seats| {
        seats[0] + 1 != seats[1] && seats[1] - 2 == seats[0]
    })
        .map(|result| result[0] + 1);

    println!("Result: {} {}", converted.last().unwrap_or(&0u32), task_two.unwrap_or(0));
}

#[cfg(test)]
mod tests {
    use crate::day_five::*;

    #[test]
    fn should_convert_binary_str_to_decimal() {
        assert_eq!(to_integer_from_binary_string("WWbeW", 'W'), Ok(25))
    }
}
