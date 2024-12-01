use adventofcode2024::AocSolution;

pub struct Solution;

impl AocSolution for Solution {
    const DAY: u8 = 1;

    fn new() -> Self {
        Self
    }

    fn part1(&self, input: &str) -> u64 {
        let mut list_1 = Vec::new();
        let mut list_2 = Vec::new();

        input
            .lines()
            .map(|line| {
                if line.is_empty() {
                    return 0;
                }

                // Split the line by 4 spaces
                let values = line.split("   ").collect::<Vec<&str>>();

                // Get the first and last values
                let first = values[0].parse::<i32>().unwrap();
                let last = values.last().unwrap().parse::<i32>().unwrap();

                list_1.push(first);
                list_2.push(last);

                let mut res = 0;
                if first > last {
                    res = first - last;
                } else {
                    res = last - first;
                }

                println!("RES: {}", res);

                /*
                let first = line
                    .chars()
                    .find(|c| c.is_numeric())
                    .and_then(|c| c.to_digit(10))
                    .unwrap();
                let last = line
                    .chars()
                    .rev()
                    .find(|c| c.is_numeric())
                    .and_then(|c| c.to_digit(10))
                    .unwrap();
                */
                res
            })
            .sum::<i32>() as u64
    }

    fn part2(&self, input: &str) -> u64 {
        input
            .lines()
            .map(|line| {
                let values = parse_line(line);

                let first = values[0];
                let last = values.last().unwrap();

                first * 10 + last
            })
            .sum::<u32>() as u64
    }
}

fn parse_line(line: &str) -> Vec<u32> {
    let line = line.to_ascii_lowercase();
    (0..line.len())
        .map(|i| &line[i..])
        .filter_map(|wnd| {
            let c = wnd.chars().next().unwrap();
            if c.is_ascii_digit() {
                return c.to_digit(10);
            }

            if wnd.starts_with("one") {
                Some(1)
            } else if wnd.starts_with("two") {
                Some(2)
            } else if wnd.starts_with("three") {
                Some(3)
            } else if wnd.starts_with("four") {
                Some(4)
            } else if wnd.starts_with("five") {
                Some(5)
            } else if wnd.starts_with("six") {
                Some(6)
            } else if wnd.starts_with("seven") {
                Some(7)
            } else if wnd.starts_with("eight") {
                Some(8)
            } else if wnd.starts_with("nine") {
                Some(9)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

adventofcode2024::run!(Solution);
