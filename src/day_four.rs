use crate::file_util::read_lines;
use std::collections::HashMap;
use itertools::Itertools;

struct Credential {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>
}

impl Credential {
    fn new(data: HashMap<String, String>) -> Self {
        Credential {
            byr: data.get("byr").map(|x| x.to_owned()),
            iyr: data.get("iyr").map(|x| x.to_owned()),
            eyr: data.get("eyr").map(|x| x.to_owned()),
            hgt: data.get("hgt").map(|x| x.to_owned()),
            hcl: data.get("hcl").map(|x| x.to_owned()),
            ecl: data.get("ecl").map(|x| x.to_owned()),
            pid: data.get("pid").map(|x| x.to_owned())
        }
    }

    fn is_valid_for_task_one(&self) -> bool {
        self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
    }
}

fn convert_to_credentials(iterator: impl Iterator<Item = String>) -> impl Iterator<Item = Credential> {
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
            Credential::new(
                line
                    .as_str()
                    .split(' ')
                    .filter_map(|part| {
                        let index = part.find(':')?;
                        Some((String::from(&part[..index]), String::from(&part[index+1..])))
                    })
                    .collect()
            )
        )
}

#[allow(dead_code)]
pub fn run_day_four() {
    let converted = convert_to_credentials(
        read_lines("assets/day_four")
    );
    let task_one = converted
        .filter(|credential| credential.is_valid_for_task_one())
        .count();

    println!("Result Task One: {}", task_one)
}

#[cfg(test)]
mod tests {
    use crate::day_four::*;

    #[test]
    fn should_convert_lines_to_credentials() {
        let under_test = vec!(String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"));
        let result = convert_to_credentials(under_test.into_iter())
            .collect::<Vec<Credential>>();
        let first_result = result.first().unwrap();

        assert_eq!(first_result.ecl.as_ref().unwrap(), &String::from("gry"));
        assert_eq!(first_result.pid.as_ref().unwrap(), &String::from("860033327"));
        assert_eq!(first_result.eyr.as_ref().unwrap(), &String::from("2020"));
        assert_eq!(first_result.hcl.as_ref().unwrap(), &String::from("#fffffd"));
        assert_eq!(first_result.byr.as_ref().unwrap(), &String::from("1937"));
        assert_eq!(first_result.iyr.as_ref().unwrap(), &String::from("2017"));
        assert_eq!(first_result.hgt.as_ref().unwrap(), &String::from("183cm"))
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
        )
            .filter(|credential| credential.is_valid_for_task_one())
            .count();
        assert_eq!(creds, 2)
    }
}
