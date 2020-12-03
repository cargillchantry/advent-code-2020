use crate::file_util::read_lines;

struct PasswordPolicy {
    at_least_length: usize,
    at_most_length: usize,
    letter: char,
    password: String
}

fn parse_password_file(lines: impl Iterator<Item = String>) -> impl Iterator<Item = PasswordPolicy> {
    lines.filter_map(|line| {
        let mut split_password = line
            .splitn(4, |split_on| split_on == ' ' || split_on == '-');

        let at_least_length = split_password.next()?.parse::<usize>().ok()?;
        let at_most_length = split_password.next()?.parse::<usize>().ok()?;
        let letter = split_password.next()?.chars().next()?;
        let password = split_password.next()?.to_owned();
        Some(
            PasswordPolicy {
                at_least_length,
                at_most_length,
                letter,
                password
            }
        )
    })
}

fn is_valid_for_task_one(policy: &PasswordPolicy) -> bool {
    let char_count = policy.password
        .chars()
        .filter(|curr| curr == &policy.letter)
        .count();
    char_count <= policy.at_most_length && char_count >= policy.at_least_length
}

fn is_valid_for_task_two(policy: &PasswordPolicy) -> bool {
    let mut iterator = policy.password.chars();
    let is_first = iterator
        .nth(policy.at_least_length - 1)
        .map_or(false, |letter| letter == policy.letter);
    let is_second = iterator
        .nth(policy.at_most_length - policy.at_least_length - 1)
        .map_or(false, |letter| letter == policy.letter);
    is_first ^ is_second
}

#[allow(dead_code)]
pub fn run_day_two() {
    let mut result = [0, 0];
    let number_valid = parse_password_file(read_lines("assets/day_two"))
        .fold(&mut result, |prev, policy| {
            if is_valid_for_task_one(&policy) {
                prev[0] += 1;
            }
            if is_valid_for_task_two(&policy) {
                prev[1] += 1;
            }
            prev
        });
    println!("Number valid: {} {}", number_valid[0], number_valid[1])
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
    fn should_reject_invalid_password_file_for_task_one_due_to_at_least_condition() {
        assert_eq!(
            is_valid_for_task_one(&PasswordPolicy{
                at_least_length: 1,
                at_most_length: 3,
                letter: 'a',
                password: "bbbb".to_owned()
            }),
            false
        )
    }

    #[test]
    fn should_reject_invalid_password_file_for_task_one_due_to_at_most_condition() {
        assert_eq!(
            is_valid_for_task_one(&PasswordPolicy{
                at_least_length: 1,
                at_most_length: 3,
                letter: 'a',
                password: "aaaabbbb".to_owned()
            }),
            false
        )
    }

    #[test]
    fn should_accept_valid_password_file_for_task_one() {
        assert_eq!(
            is_valid_for_task_one(&PasswordPolicy{
                at_least_length: 1,
                at_most_length: 3,
                letter: 'a',
                password: "a".to_owned()
            }),
            true
        );
        assert_eq!(
            is_valid_for_task_one(&PasswordPolicy{
                at_least_length: 1,
                at_most_length: 3,
                letter: 'a',
                password: "aaabbbb".to_owned()
            }),
            true
        )
    }

    #[test]
    fn should_accept_valid_password_file_for_task_two() {
        assert_eq!(
            is_valid_for_task_two(&PasswordPolicy{
                at_least_length: 1,
                at_most_length: 3,
                letter: 'a',
                password: "abc".to_owned()
            }),
            true
        );
        assert_eq!(
            is_valid_for_task_two(&PasswordPolicy{
                at_least_length: 2,
                at_most_length: 3,
                letter: 'a',
                password: "dbaf".to_owned()
            }),
            true
        )
    }

    #[test]
    fn should_reject_invalid_password_file_due_to_both_matching_for_task_two() {
        assert_eq!(
            is_valid_for_task_two(&PasswordPolicy{
                at_least_length: 4,
                at_most_length: 9,
                letter: 'b',
                password: "aaabaaaaba".to_owned()
            }),
            false
        );
    }

    #[test]
    fn should_reject_invalid_password_file_due_to_neither_matching_for_task_two() {
        assert_eq!(
            is_valid_for_task_two(&PasswordPolicy{
                at_least_length: 4,
                at_most_length: 9,
                letter: 'b',
                password: "a".to_owned()
            }),
            false
        );
    }
}
