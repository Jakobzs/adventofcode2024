use adventofcode2024::AocSolution;

pub struct Solution;

impl AocSolution for Solution {
    const DAY: u8 = 2;

    fn new() -> Self {
        Self
    }

    fn part1<T: AsRef<str>>(&self, input: T) -> u64 {
        count_safe_reports(input) as u64
    }

    fn part2<T: AsRef<str>>(&self, input: T) -> u64 {
        count_safe_reports_pd(input) as u64
    }
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return false; // A report must have at least two levels to check.
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] - window[0];

        if diff < -3 || diff > 3 || diff == 0 {
            return false; // The difference between adjacent levels must be between 1 and 3.
        }

        if diff < 0 {
            increasing = false;
        } else if diff > 0 {
            decreasing = false;
        }
    }

    // A report is safe if it's entirely increasing or entirely decreasing.
    increasing || decreasing
}

fn count_safe_reports<T: AsRef<str>>(input: T) -> usize {
    input
        .as_ref()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .filter(|report| is_safe_report(report))
        .count()
}

fn count_safe_reports_pd<T: AsRef<str>>(input: T) -> usize {
    input
        .as_ref()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .filter(|report| is_safe_with_dampener(report))
        .count()
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    // First, check if the report is already safe.
    if is_safe_report(report) {
        return true;
    }

    // If not, try removing each level and check if the result is safe.
    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i); // Remove the level at index `i`.

        if is_safe_report(&modified_report) {
            return true; // If removing this level makes it safe, the report is safe.
        }
    }

    false
}

adventofcode2024::run!(Solution);
