use itertools::Itertools;
use crate::file_util::read_lines_as_u32;

fn find_pair_summing_to(numbers: &[u32], value: u32) -> Option<(&u32, &u32)> {
    numbers.iter().tuple_combinations()
        .find(|(first, second)| *first + *second == value)
}

fn find_triple_summing_to(numbers: &[u32], value: u32) -> Option<(&u32, &u32, &u32)> {
    numbers.iter().tuple_combinations()
        .find(|(first, second, third)| *first + *second + *third == value)
}

#[allow(dead_code)]
pub fn run_day_one() {
    let numbers = read_lines_as_u32("assets/day_one").collect_vec();
    let first_result = find_pair_summing_to(&numbers, 2020);
    let second_result = find_triple_summing_to(&numbers, 2020);
    match first_result {
        None => println!("No match!"),
        Some(products) => println!("Result: {}", products.0 * products.1)
    }
    match second_result {
        None => println!("No match!"),
        Some(products) => println!("Result: {}", products.0 * products.1 * products.2)
    }
}


#[cfg(test)]
mod tests {
    use crate::day_one::*;

    #[test]
    fn should_produce_empty_if_none_sum_for_pair() {
        assert_eq!(find_pair_summing_to(&vec!(1, 2, 3, 4), 12), None)
    }

    #[test]
    fn should_produce_pair_if_sum_for_pair() {
        assert_eq!(find_pair_summing_to(&vec!(1, 2, 3, 4), 7), Some((&3, &4)))
    }

    #[test]
    fn should_produce_empty_if_none_sum_for_triple() {
        assert_eq!(find_triple_summing_to(&vec!(1, 2, 3, 4), 12), None)
    }

    #[test]
    fn should_produce_pair_if_sum() {
        assert_eq!(find_triple_summing_to(&vec!(1, 2, 3, 4), 9), Some((&2, &3, &4)))
    }
}
