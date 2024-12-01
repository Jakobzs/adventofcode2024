use adventofcode2024::AocSolution;
use std::collections::HashMap;

pub struct Solution;

impl AocSolution for Solution {
    const DAY: u8 = 1;

    fn new() -> Self {
        Self
    }

    fn part1<T: AsRef<str>>(&self, input: T) -> u64 {
        // Get the lists of numbers from the input
        let (mut left_list, mut right_list) = get_lists(input);

        // Sort the lists.
        left_list.sort();
        right_list.sort();

        // Iterate over the lists and subtract the element from list 2 from list 1
        let total_distance: i32 = left_list
            .iter()
            .zip(right_list.iter())
            .map(|(left, right)| (left - right).abs())
            .sum();

        total_distance as u64
    }

    fn part2<T: AsRef<str>>(&self, input: T) -> u64 {
        let (left_list, right_list) = get_lists(input);

        // Create a frequency map for the right list
        let mut right_frequency = HashMap::new();
        for num in &right_list {
            *right_frequency.entry(num).or_insert(0) += 1;
        }

        // Calculate the similarity score
        let mut similarity_score = 0;
        for num in &left_list {
            if let Some(count) = right_frequency.get(&num) {
                similarity_score += num * count;
            }
        }

        similarity_score as u64
    }
}

fn get_lists<T: AsRef<str>>(input: T) -> (Vec<i32>, Vec<i32>) {
    input
        .as_ref()
        .lines()
        .map(|line| {
            let mut values = line.split("   ").map(|v| v.parse::<i32>().unwrap());
            (values.next().unwrap(), values.next().unwrap())
        })
        .unzip()
}

adventofcode2024::run!(Solution);
