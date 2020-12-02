use crate::file_util::read_lines;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug)]
struct PasswordPolicy {
    at_least_length: usize,
    at_most_length: usize,
    letter: char,
    password: String
}

fn parse_password_file(lines: impl Iterator<Item = String>) -> impl Iterator<Item = PasswordPolicy> {
    lines.filter_map(|line| {
        let mut split_password = line
            .splitn(4, |char: char| char == ' ' || char == '-')
            .into_iter();
        let at_least_length_part = split_password.next();
        let at_most_length_part = split_password.next();
        let letter_part = split_password.next().and_then(|str| str.chars().next());
        let password_part = split_password.next();
        at_least_length_part
            .zip(at_most_length_part)
            .zip(letter_part.zip(password_part))
            .map(|((at_least_length, at_most_length), (letter, password))| PasswordPolicy {
                at_least_length: usize::from_str(at_least_length).expect("Malformed file input."),
                at_most_length: usize::from_str(at_most_length).expect("Malformed file input."),
                letter,
                password: password.to_owned()
            })
    })
}

fn is_valid(policy: &PasswordPolicy) -> bool {
    let char_count = policy.password
        .chars()
        .filter(|curr| curr == &policy.letter)
        .count();
    char_count <= policy.at_most_length && char_count >= policy.at_least_length
}

#[allow(dead_code)]
pub fn run_day_two() {
    let number_valid = parse_password_file(read_lines("assets/day_two"))
        .filter(is_valid)
        .count();
    print!("Number valid: {}", number_valid)
}


#[cfg(test)]
mod tests {
    use crate::day_two::*;
    use itertools::Itertools;

    #[test]
    fn should_parse_valid_password_file() {
        let parsed = parse_password_file(
            vec!("3-4 q: wqqkzwqgkqkk".to_owned(), "1-2 k: aaa".to_owned(), "1-2 a: aaa".to_owned()).into_iter()
        ).collect_vec();
        let first = parsed.last();
        assert_eq!(parsed.len(), 3);
        assert_eq!(first.map(|p| p.password.as_str()).get_or_insert(""), &"aaa");
        assert_eq!(first.map(|p| p.letter).get_or_insert(' '), &'a');
        assert_eq!(first.map(|p| p.at_least_length).get_or_insert(0), &mut 1);
        assert_eq!(first.map(|p| p.at_most_length).get_or_insert(0), &mut 2)
    }

    #[test]
    fn should_reject_invalid_password_file_due_to_at_least_condition() {
        assert_eq!(
            is_valid(&PasswordPolicy{
                at_least_length: 1,
                at_most_length: 3,
                letter: 'a',
                password: "bbbb".to_owned()
            }),
            false
        )
    }

    #[test]
    fn should_reject_invalid_password_file_due_to_at_most_condition() {
        assert_eq!(
            is_valid(&PasswordPolicy{
                at_least_length: 1,
                at_most_length: 3,
                letter: 'a',
                password: "aaaabbbb".to_owned()
            }),
            false
        )
    }

    #[test]
    fn should_accept_valid_password_file() {
        assert_eq!(
            is_valid(&PasswordPolicy{
                at_least_length: 1,
                at_most_length: 3,
                letter: 'a',
                password: "a".to_owned()
            }),
            true
        );
        assert_eq!(
            is_valid(&PasswordPolicy{
                at_least_length: 1,
                at_most_length: 3,
                letter: 'a',
                password: "aaabbbb".to_owned()
            }),
            true
        )
    }
}
