use crate::file_util::read_lines;
use std::collections::{HashMap};
use crate::day_nineteen::ProductionRule::*;
use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
enum ProductionRule {
    TerminalSymbol(char),
    Reference(usize),
    And(Box<ProductionRule>, Box<ProductionRule>),
    Or(Box<ProductionRule>, Box<ProductionRule>)
}

impl ProductionRule {
    fn expand<'a>(&self, buffer: &mut String, lookup: &impl Fn(usize) -> &'a ProductionRule) {
        match self {
            TerminalSymbol(x) => {
                buffer.push(*x)
            },
            Reference(x) => lookup(*x).expand(buffer, lookup),
            And(x, y) => {
                x.expand(buffer, lookup);
                y.expand(buffer, lookup)
            },
            Or(x, y) => {
                buffer.push('(');
                x.expand(buffer, lookup);
                buffer.push('|');
                y.expand(buffer, lookup);
                buffer.push(')');
            }
        }
    }
}

#[allow(dead_code)]
pub fn run_day_nineteen() {
    let mut input_iter = read_lines("assets/day_nineteen");
    let grammar_rules = parse_grammar_into_rules(
        &mut input_iter.by_ref().take_while(|it| !it.trim().is_empty())
    );
    let regex = Regex::new(
        &format!("^{}$", convert_grammar_rules_to_regex(&grammar_rules).unwrap())
    )
        .unwrap();
    let matching_sentences = input_iter
        .filter(|x| !x.trim().is_empty())
        .filter(|x| regex.is_match(&x))
        .count();
    println!("Result Task 1 {:?}", matching_sentences);
}

fn convert_grammar_rules_to_regex(rules: &HashMap<usize, ProductionRule>) -> Option<String> {
    let start = rules.get(&0_usize)?;
    let mut result = String::new();
    start.expand(&mut result, &|x| rules.get(&x).unwrap());
    Some(result)
}

fn parse_grammar_into_rules(iter: &mut impl Iterator<Item = String>) -> HashMap<usize, ProductionRule>{
    let mut rules = HashMap::new();
    for line in iter {
        let mut split = line.splitn(2, ':');
        if let Some((rule, rest)) = split.next()
            .and_then(|it| it.parse::<usize>().ok())
            .zip(split.next()) {
            if rest.contains('"') {
                let terminal = rest.chars()
                    .skip_while(|it| *it != '"')
                    .nth(1);
                if let Some(character) = terminal {
                    rules.insert(rule, TerminalSymbol(character));
                }
            } else {
                let mut split = rest.split('|')
                    .filter_map(|sub_rule| {
                        let productions = sub_rule.split(' ')
                            .filter_map(|it| it.trim().parse::<usize>().ok())
                            .collect::<Vec<usize>>();
                        if !productions.is_empty() {
                            Some(
                                productions.iter().skip(1)
                                    .fold(
                                        Reference(productions[0]),
                                        |prev, production| {
                                            And(
                                                Box::new(prev),
                                                Box::new(Reference(*production))
                                            )
                                        }
                                    )
                            )
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<ProductionRule>>();
                if let Some(last) = split.pop() {
                    rules.insert(
                        rule,
                        split.into_iter().rev().fold(last, |prev, next| {
                            Or(Box::new(next), Box::new(prev))
                        })
                    );
                }
            }
        }
    }
    rules
}

#[cfg(test)]
mod tests {
    use crate::day_nineteen::*;

    #[test]
    fn should_recognise_valid_strings() {
        let mut to_process = vec!(
            String::from("0: 4 1 5"),
            String::from("1: 2 3 | 3 2"),
            String::from("2: 4 4 | 5 5"),
            String::from("3: 4 5 | 5 4"),
            String::from("4: \"a\""),
            String::from("5: \"b\""),
            String::from(""),
            String::from("ababbb"),
            String::from("bababa"),
            String::from("abbbab"),
            String::from("aaabbb"),
            String::from("aaaabbb")
        ).into_iter();

        let grammar_rules = parse_grammar_into_rules(
            &mut to_process.by_ref().take_while(|it| !it.trim().is_empty())
        );
        let regex = Regex::new(
            &format!("^{}$", convert_grammar_rules_to_regex(&grammar_rules).unwrap())
        ).unwrap();
        let matching_sentences = to_process
            .filter(|x| !x.trim().is_empty())
            .filter(|x| regex.is_match(&x))
            .count();
        assert_eq!(matching_sentences, 2);
    }
}
