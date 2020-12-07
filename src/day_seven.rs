use crate::file_util::read_non_blank_lines;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

struct Bag {
    parents: HashMap<String, u16>,
    children: Vec<(String, u16)>
}

fn search_parents<'a>(bag: &Bag, tree: &HashMap<String, Bag>, parents: &'a mut HashSet<String>) -> &'a HashSet<String> {
    for (key, _) in bag.parents.iter() {
        if !parents.contains(key) {
            parents.insert(key.clone());
            if let Some(parent_bag) = tree.get(key) {
                search_parents(
                    parent_bag,
                    tree,
                    parents
                );
            }
        }
    }

    parents
}

fn get_children_count(bag: &Bag, tree: &HashMap<String, Bag>) -> usize {
    bag.children.iter()
        .map(|(child, count)|
            usize::from(*count) * (
                1 + tree
                    .get(child)
                    .map(|child_bag| get_children_count(child_bag, tree))
                    .unwrap_or(0)
            )
        )
        .sum()
}

#[allow(dead_code)]
pub fn run_day_seven() {
    let mut bags: HashMap<String, Bag> = HashMap::new();

    read_non_blank_lines("assets/day_seven").for_each(|line| {
        let mut words = line.split(' ');
        let name = words.by_ref().take_while(|word| *word != "bags").join(" ");
        words.by_ref().find(|word| *word != "no");

        let children = words.batching(|children| {
            let blah = children.next();
            if let Some(count) = blah.and_then(|c| u16::from_str(c).ok()) {
                let bag_name = children
                    .take_while(|word| !word.starts_with("bag"))
                    .join(" ");
                Some((bag_name, count))
            } else {
                None
            }
        }).collect::<Vec<(String, u16)>>();

        children.iter().for_each(|(bag_name, count)|
            if let Some(bag) = bags.get_mut(bag_name) {
                bag.parents.insert(name.clone(), *count);
            } else {
                let mut parents = HashMap::new();
                parents.insert(name.clone(), *count);
                bags.insert(bag_name.clone(), Bag { parents, children: Vec::new() });
            }
        );

        if let Some(parent) = bags.get_mut(&name) {
            parent.children = children;
        } else {
            bags.insert(name, Bag { parents: HashMap::new(), children });
        }
    });

    let mut result = HashSet::new();
    if let Some(shiny_gold) = bags.get("shiny gold") {
        search_parents(shiny_gold, &bags, &mut result);
    }
    let result2 = bags.get("shiny gold").map(|b| get_children_count(b, &bags)).unwrap_or(0);
    println!("Result: {} {}", result.len(), result2);
}

#[cfg(test)]
mod tests {
    use crate::day_seven::*;

    /**
    * Struggled with time ...
    */
    #[test]
    fn should_() {}
}
