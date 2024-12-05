use std::collections::{HashMap, HashSet};

use adventofcode2024::AocSolution;

pub struct Solution;

impl AocSolution for Solution {
    const DAY: u8 = 5;

    fn new() -> Self {
        Self
    }

    fn part1<T: AsRef<str>>(&self, input: T) -> u64 {
        let (rules, updates) = parse_input(input.as_ref());

        validate_updates(&rules, &updates) as u64
    }

    fn part2<T: AsRef<str>>(&self, input: T) -> u64 {
        let (rules, updates) = parse_input(input.as_ref());

        reorder_updates(&rules, &updates) as u64
    }
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut sections = input.trim().split("\n\n");
    let rules_section = sections.next().unwrap();
    let updates_section = sections.next().unwrap();

    let mut rules = HashMap::new();
    for line in rules_section.lines() {
        let parts: Vec<i32> = line.split('|').map(|x| x.parse().unwrap()).collect();
        rules
            .entry(parts[0])
            .or_insert_with(Vec::new)
            .push(parts[1]);
    }

    let updates = updates_section
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn validate_updates(rules: &HashMap<i32, Vec<i32>>, updates: &[Vec<i32>]) -> i32 {
    let mut total = 0;

    for update in updates {
        if is_valid_update(rules, update) {
            let middle_index = update.len() / 2;
            total += update[middle_index];
        }
    }

    total
}

fn reorder_updates(rules: &HashMap<i32, Vec<i32>>, updates: &[Vec<i32>]) -> i32 {
    let mut total = 0;

    for update in updates {
        if !is_valid_update(rules, update) {
            let reordered_update = reorder_update(rules, update);
            let middle_index = reordered_update.len() / 2;
            total += reordered_update[middle_index];
        }
    }

    total
}

fn is_valid_update(rules: &HashMap<i32, Vec<i32>>, update: &[i32]) -> bool {
    let update_set: HashSet<i32> = update.iter().cloned().collect();

    for (&from, to_list) in rules {
        if update_set.contains(&from) {
            for &to in to_list {
                if update_set.contains(&to) {
                    if update.iter().position(|&x| x == from).unwrap()
                        > update.iter().position(|&x| x == to).unwrap()
                    {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn reorder_update(rules: &HashMap<i32, Vec<i32>>, update: &[i32]) -> Vec<i32> {
    let mut sorted_update = update.to_vec();
    sorted_update.sort_by(|a, b| {
        if rules.contains_key(a) && rules[a].contains(b) {
            std::cmp::Ordering::Less
        } else if rules.contains_key(b) && rules[b].contains(a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    sorted_update
}

adventofcode2024::run!(Solution);
