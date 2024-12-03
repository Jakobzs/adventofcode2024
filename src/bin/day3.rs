use adventofcode2024::AocSolution;
use regex::Regex;

pub struct Solution;

impl AocSolution for Solution {
    const DAY: u8 = 3;

    fn new() -> Self {
        Self
    }

    fn part1<T: AsRef<str>>(&self, input: T) -> u64 {
        // Define a regex pattern to match valid mul(X,Y) instructions
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        // Initialize the total sum
        let mut total = 0;

        // Iterate over all matches
        for cap in re.captures_iter(input.as_ref()) {
            // Parse the captured groups as integers
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();

            // Multiply and add to the total
            total += x * y;
        }

        println!("The total sum of valid multiplications is: {}", total);

        total as u64
    }

    fn part2<T: AsRef<str>>(&self, input: T) -> u64 {
        // Regular expressions for the instructions
        let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let do_regex = Regex::new(r"do\(\)").unwrap();
        let dont_regex = Regex::new(r"don't\(\)").unwrap();

        let mut is_enabled = true; // `mul` is enabled at the start
        let mut sum = 0;

        // Track the current position in the input string
        let input = input.as_ref();
        let mut current_pos = 0;

        while current_pos < input.len() {
            // Check for `do()` and `don't()` instructions
            if let Some(mat) = do_regex.find(&input[current_pos..]) {
                if current_pos + mat.start() == current_pos {
                    is_enabled = true;
                    current_pos += mat.end();
                    continue;
                }
            }
            if let Some(mat) = dont_regex.find(&input[current_pos..]) {
                if current_pos + mat.start() == current_pos {
                    is_enabled = false;
                    current_pos += mat.end();
                    continue;
                }
            }

            // Check for `mul(x, y)` instructions
            if let Some(cap) = mul_regex.captures(&input[current_pos..]) {
                let start = current_pos + cap.get(0).unwrap().start();
                let end = current_pos + cap.get(0).unwrap().end();

                if current_pos == start {
                    if is_enabled {
                        let x: u64 = cap[1].parse().unwrap();
                        let y: u64 = cap[2].parse().unwrap();
                        sum += x * y;
                    }
                    current_pos = end;
                    continue;
                }
            }

            // Move to the next character if no instruction matched
            current_pos += 1;
        }

        sum
    }
}

adventofcode2024::run!(Solution);
