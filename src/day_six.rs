use crate::file_util::read_lines;
use std::collections::{HashSet, HashMap};

struct BatchIterator<T, F> {
    iterator: T,
    transform: F
}

impl<I, F> BatchIterator<I, F> {
    fn new(iterator: I, transform: F) -> Self {
        BatchIterator {
            iterator, transform
        }
    }
}

impl<T, F, S> Iterator for BatchIterator<T, F> where T: Iterator, F: Fn(&mut T) -> Option<S>  {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        (self.transform)(&mut self.iterator)
    }
}

fn aggregate_answers(iterator: &mut impl Iterator<Item = String>) -> Option<(HashSet<char>, HashSet<char>)> {
    let mut combined: HashMap<char, usize> = HashMap::new();
    let mut count = 0;
    for line in iterator {
        if line.is_empty() {
            break;
        }
        count += 1;
        line.chars().for_each(|x| {
            combined.insert(x, combined.get(&x).map_or(1, |y| y + 1));
        });
    }
    if count == 0 {
        None
    } else {
        Some(
            (
                combined.keys().copied().collect(),
                combined.iter()
                    .filter_map(|(x, y)| if *y == count { Some(*x) } else { None })
                    .collect()
            )
        )
    }
}

#[allow(dead_code)]
pub fn run_day_six() {
    let mut result = 0;
    let mut result_two = 0;
    BatchIterator::new(
        read_lines("assets/day_six"),
        aggregate_answers
    )
        .for_each(|(first, second)| {
            result += first.len();
            result_two += second.len();
        });

    println!("Result: {} {}", result, result_two);
}

#[cfg(test)]
mod tests {
    use crate::day_six::*;
    use itertools::__std_iter::FromIterator;

    #[test]
    fn should_count_answers_in_groups() {
        let answers: Vec<(HashSet<char>, HashSet<char>)> = BatchIterator::new(vec!(
            String::from("ab"),
            String::from("bc"),
            String::from("d"),
            String::from(""),
            String::from("a"),
            String::from("ab")
        ).into_iter(), aggregate_answers).collect();
        let first = answers.first().unwrap();
        let second = answers.last().unwrap();
        assert_eq!(answers.len(), 2);
        assert_eq!(first.0, HashSet::from_iter(vec!('a', 'b', 'c', 'd').into_iter()));
        assert_eq!(first.1, HashSet::new());
        assert_eq!(second.0, HashSet::from_iter(vec!('a', 'b').into_iter()));
        assert_eq!(second.1, HashSet::from_iter(vec!('a').into_iter()))
    }
}
