use crate::file_util::read_lines;
use std::collections::HashMap;
use itertools::Itertools;
use std::str::FromStr;

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

struct Credentials {
    credentials: Vec<Credential>
}

impl Credentials {
    fn new(data: HashMap<String, String>) -> Credentials {
        Credentials {
            credentials: data.iter()
                .filter_map(|(key, value)| match key.as_str() {
                    "byr" => Some(Credential::BYR(value.to_owned())),
                    "iyr" => Some(Credential::IYR(value.to_owned())),
                    "eyr" => Some(Credential::EYR(value.to_owned())),
                    "hgt" => Some(Credential::HGT(value.to_owned())),
                    "hcl" => Some(Credential::HCL(value.to_owned())),
                    "ecl" => Some(Credential::ECL(value.to_owned())),
                    "pid" => Some(Credential::PID(value.to_owned())),
                    _ => None
                })
                .collect()
        }
    }

    fn is_valid_for_task_one(&self) -> bool {
        self.credentials.len() == 7
    }

    fn is_valid_for_task_two(&self) -> bool {
        self.is_valid_for_task_one() &&
            !self.credentials.iter().find(|c| !c.is_valid()).is_some()
    }
}

#[derive(Eq, PartialEq)]
enum Credential {
    BYR(String),
    IYR(String),
    EYR(String),
    HGT(String),
    HCL(String),
    ECL(String),
    PID(String)
}

impl Credential {
    fn is_valid(&self) -> bool {
        match self {
            Credential::EYR(v) => is_number_between(v, 2020, 2030),
            Credential::IYR(v) => is_number_between(v, 2010, 2020),
            Credential::BYR(v) => is_number_between(v, 1920, 2002),
            Credential::HCL(v) => v.starts_with('#')
                && v.len() == 7
                && v.chars().skip(1).find(|c| !c.is_ascii_hexdigit()).is_none(),
            Credential::HGT(v) => if v.ends_with("cm") {
                is_number_between(&v[..v.len()-2], 150, 193)
            } else if v.ends_with("in") {
                is_number_between(&v[..v.len()-2], 59, 76)
            } else {
                false
            },
            Credential::ECL(v) => EYE_COLORS.contains(&v.as_str()),
            Credential::PID(v) => v.chars().find(|c| !c.is_numeric()).is_none()
                && v.len() == 9
        }
    }
}

fn is_number_between(value: &str, start: u16, end: u16) -> bool {
    u16::from_str(value).ok()
        .filter(|it| *it >= start && *it <= end)
        .is_some()
}

fn convert_to_credentials(iterator: impl Iterator<Item = String>) -> impl Iterator<Item =Credentials> {
    iterator
        .batching(|iterator| {
            let next = iterator
                .take_while(|value| !value.is_empty())
                .join(" ");
            if next.is_empty() {
                None
            } else {
                Some(next)
            }
        })
        .map(|line|
            Credentials::new(
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
    let mut results = [0, 0];
    converted.for_each(|credentials| {
        if credentials.is_valid_for_task_one() {
            results[0] += 1
        }
        if credentials.is_valid_for_task_two() {
            results[1] += 1
        }
    });

    println!("Result: {} {}", results[0], results[1])
}

#[cfg(test)]
mod tests {
    use crate::day_four::*;

    #[test]
    fn should_convert_lines_to_credentials() {
        let under_test = vec!(String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"));
        let result = convert_to_credentials(under_test.into_iter())
            .collect::<Vec<Credentials>>();
        let first_result = result.first().unwrap();


        assert_eq!(first_result.credentials.contains(&Credential::ECL("gry".into())), true);
        assert_eq!(first_result.credentials.contains(&Credential::PID("860033327".into())), true);
        assert_eq!(first_result.credentials.contains(&Credential::EYR("2020".into())), true);
        assert_eq!(first_result.credentials.contains(&Credential::HCL("#fffffd".into())), true);
        assert_eq!(first_result.credentials.contains(&Credential::BYR("1937".into())), true);
        assert_eq!(first_result.credentials.contains(&Credential::IYR("2017".into())), true);
        assert_eq!(first_result.credentials.contains(&Credential::HGT("183cm".into())), true)
    }

    #[test]
    fn should_count_valid_credentials_for_task_one() {
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
