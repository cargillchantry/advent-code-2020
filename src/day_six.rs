use crate::file_util::read_lines;
use std::collections::HashSet;

struct GroupIterator<T, F> {
    iterator: T,
    transform: F
}

impl<I, F> GroupIterator<I, F> {
    fn new(iterator: I, transform: F) -> Self {
        GroupIterator {
            iterator, transform
        }
    }
}

impl<T, F, S> Iterator for GroupIterator<T, F> where T: Iterator, F: Fn(&mut T) -> Option<S>  {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        (self.transform)(&mut self.iterator)
    }
}

fn aggregate_answers(iterator: &mut impl Iterator<Item = String>) -> Option<HashSet<char>> {
    let mut aggregation = HashSet::new();
    while let Some(line) = iterator.next() {
        if line.is_empty() {
            break;
        }
        line.chars().for_each(|letter| { aggregation.insert(letter); });
    }
    if aggregation.is_empty() {
        None
    } else {
        Some(aggregation)
    }
}

#[allow(dead_code)]
pub fn run_day_six() {
    let result: usize = GroupIterator::new(
        read_lines("assets/day_six"),
        aggregate_answers
    )
        .map(|group_answers| group_answers.len())
        .sum();

    println!("Result: {} {}", result, 0);
}

#[cfg(test)]
mod tests {
    use crate::day_six::*;
    use itertools::__std_iter::FromIterator;

    #[test]
    fn should_count_answers_in_groups() {
        let answers: Vec<HashSet<char>> = GroupIterator::new(vec!(
            String::from("ab"),
            String::from("bc"),
            String::from("d"),
            String::from(""),
            String::from("a")
        ).into_iter(), aggregate_answers).collect();

        assert_eq!(answers.len(), 2);
        assert_eq!(answers.first().unwrap(), &HashSet::from_iter(vec!('a', 'b', 'c', 'd').into_iter()));
        assert_eq!(answers.last().unwrap(), &HashSet::from_iter(vec!('a').into_iter()))
    }
}
