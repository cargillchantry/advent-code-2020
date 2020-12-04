use crate::file_util::read_lines;
use std::collections::HashMap;
use itertools::Itertools;

const MANDITORY_FIELDS: &[&str] = &[
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid"
];

fn convert_to_credentials(iterator: impl Iterator<Item = String>) -> impl Iterator<Item = HashMap<String, String>> {
    iterator
        .batching(|iterator| {
            let mut buffer = Vec::new();
            loop {
                let next = iterator.next()
                    .filter(|value| !value.is_empty());
                if let Some(value) = next {
                    buffer.push(value);
                } else {
                    return if buffer.is_empty() {
                        None
                    } else {
                        Some(buffer.join(" "))
                    }
                }
            }
        })
        .map(|line|
            line
                .as_str()
                .split(' ')
                .filter_map(|part| {
                    let index = part.find(':')?;
                    Some((String::from(&part[..index]), String::from(&part[index+1..])))
                })
                .collect()
        )
}

fn count_valid(iterator: impl Iterator<Item = HashMap<String, String>>) -> usize {
    iterator
        .filter(|credential|
            MANDITORY_FIELDS.iter().all(|key| credential.contains_key(*key))
        )
        .count()
}

#[allow(dead_code)]
pub fn run_day_four() {
    let converted = convert_to_credentials(
        read_lines("assets/day_four")
    );

    println!("Result Task One: {}", count_valid(converted))
}

#[cfg(test)]
mod tests {
    use crate::day_four::*;

    #[test]
    fn should_convert_lines_to_credentials() {
        let under_test = vec!(String::from("test:value another:again"));
        let result = convert_to_credentials(under_test.into_iter())
            .collect::<Vec<HashMap<String, String>>>();
        let first_result = result.first().unwrap();

        assert_eq!(first_resut.get("test").unwrap(), &String::from("value"));
        assert_eq!(first_resut.get("another").unwrap(), &String::from("again"))
    }

    #[test]
    fn should_count_valid_credentials() {
        let under_test = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in";
        let creds = convert_to_credentials(
            under_test.lines().map(|x| String::from(x.trim())).into_iter()
        );
        assert_eq!(count_valid(creds), 2)
    }
}
