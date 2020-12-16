use crate::file_util::read_non_blank_lines;
use std::str::FromStr;
use std::collections::HashSet;
use itertools::Itertools;

/**
*   Just typed this out and ran. Would like to refactor, but no time.
*/

type Rule = [[u16; 2]; 2];

#[allow(dead_code)]
pub fn run_day_sixteen() {
    let mut lines = read_non_blank_lines("assets/day_sixteen");
    let rules = read_rules(&mut lines);
    let your_ticket = lines.next().map(|x| parse_ticket(x.as_str()));
    lines.next();
    let mut other_tickets: Vec<Vec<u16>> = lines
        .map(|x| parse_ticket(x.as_str()))
        .collect();
    let bad_tickets: Vec<(usize, u16)> = get_bad_tickets(&other_tickets, &rules);
    let part_one: usize = bad_tickets.iter().map(|t| t.1 as usize).sum();
    for (index, _) in bad_tickets.iter().rev() {
        other_tickets.remove(*index);
    }
    println!("Part One: {}", part_one);

    let mut index_hashes: Vec<HashSet<usize>> = rules.iter().map(|rule| {
        let mut possible_columns = (0..rules.len()).collect::<HashSet<usize>>();
        for ticket in other_tickets.iter() {
            for (index, value) in ticket.iter().enumerate() {
                if !satisfies_rule(*value, rule) {
                    possible_columns.remove(&index);
                    if possible_columns.len() == 1 {
                        return possible_columns;
                    }
                }
            }
        }
        possible_columns
    }).collect();

    while index_hashes.iter().any(|x| x.len() > 1) {
        for i in 0 .. rules.len() {
            let indexes = index_hashes.iter()
                .enumerate()
                .filter(|(_, x)| x.contains(&i))
                .map(|(idx, _)| idx)
                .collect::<Vec<usize>>();
            if indexes.len() == 1 {
                index_hashes[indexes[0]].clear();
                index_hashes[indexes[0]].insert(i);
            }
        }
        let to_clear = index_hashes.iter()
            .filter(|y| y.len() == 1)
            .map(|x| *x.iter().next().unwrap())
            .collect_vec();
        for v in to_clear.iter() {
            index_hashes.iter_mut().for_each(|z| {
               if z.len() > 1 {
                   z.remove(v);
               }
            });
        }
    }

    if let Some(ticket) = your_ticket {
        println!(
            "Part Two: {}",
            index_hashes.iter().take(6).filter_map(|x|  ticket.get(*x.iter().next().unwrap()).copied())
                .map(|it| it as usize)
                .product::<usize>()
        );
    }

}

fn get_bad_tickets(tickets: &[Vec<u16>], rules: &[Rule]) -> Vec<(usize, u16)> {
    tickets
        .iter()
        .enumerate()
        .filter_map(|(index, ticket)| {
            ticket.iter()
                .find(|ticket|
                    !rules.iter().any(|rule| satisfies_rule(**ticket, rule))
                )
                .map(|x| (index, *x))
        })
        .collect()
}

fn satisfies_rule(v: u16, rule: &Rule) -> bool {
    v >= rule[0][0] && v <= rule[0][1] || v >= rule[1][0] && v <= rule[1][1]
}

fn parse_ticket(ticket: &str) -> Vec<u16> {
    ticket.split(',').filter_map(|t| u16::from_str(t).ok()).collect()
}

fn read_rules(iter: &mut impl Iterator<Item = String>) -> Vec<Rule> {
    iter.take_while(|l| !l.starts_with("your ticket:"))
        .filter_map(|line| {
            let mut split_line = line.split(' ').rev();
            let first = split_line.next();
            split_line.next();
            let second = split_line.next();
            Some([
                range_to_arr_tuple(second?)?,
                range_to_arr_tuple(first?)?
            ])
        })
        .collect()
}

fn range_to_arr_tuple(range: &str) -> Option<[u16; 2]> {
    range.chars()
        .enumerate()
        .find_map(|(i, v)| if v == '-' { Some(i) } else { None })
        .map(|x| range.split_at(x))
        .and_then(|(x, y)| Some([
            u16::from_str(x).ok()?,
            u16::from_str(&y[1..]).ok()?
        ]))
}

#[cfg(test)]
mod tests {
    use crate::day_sixteen::*;

    #[test]
    fn should_read_rules() {
        let result = read_rules(
            &mut vec!(
                String::from("type: 32-55 or 65-968"),
                String::from("wagon: 39-642 or 660-955"),
                String::from("zone: 41-567 or 578-959"),
                String::from("your ticket:")
            ).into_iter()
        );
        assert_eq!(
            result,
            [
                [[32u16, 55u16], [65u16, 968u16]],
                [[39u16, 642u16], [660u16, 955u16]],
                [[41u16, 567u16], [578u16, 959u16]]
            ]
        )
    }
}
