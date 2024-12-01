use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use std::path::PathBuf;

pub trait AocSolution {
    const DAY: u8;
    fn new() -> Self
    where
        Self: Sized;
    fn part1<T: AsRef<str>>(&self, input: T) -> u64;
    fn part2<T: AsRef<str>>(&self, input: T) -> u64;
}

#[derive(Parser)]
struct Cli {
    /// Download input and run solution.
    day: Option<u8>,
}

async fn get_input(day: u8) -> Result<String> {
    // Get the path to the input file.
    let path = PathBuf::from(format!("input/day{day:02}.txt"));

    // Create the directory if it doesn't exist.
    tokio::fs::create_dir_all(path.parent().unwrap()).await?;

    // Check if the file exists and return its contents if it does.
    if tokio::fs::try_exists(&path).await? {
        return Ok(tokio::fs::read_to_string(&path).await?);
    }

    // Get the session cookie which is stored in a file called `.cookie`.
    let cookie = include_str!("../.cookie").trim();

    // Download the input from the Advent of Code website.
    let body = Client::new()
        .get(format!("https://adventofcode.com/2024/day/{day}/input"))
        .header("Cookie", cookie)
        .send()
        .await?
        .text()
        .await?;

    // Write the input to the file.
    tokio::fs::write(path, &body).await?;

    Ok(body)
}

pub async fn run_solution<S: AocSolution>() {
    let input = get_input(S::DAY).await.unwrap();
    let solution = S::new();

    let p1 = solution.part1(&input);
    println!("\x1b[32;1mPart 1:\x1b[33;1m {p1}\x1b[0m");

    let p2 = solution.part2(&input);
    println!("\x1b[32;1mPart 2:\x1b[33;1m {p2}\x1b[0m");
}

#[macro_export]
macro_rules! run {
    ($sol:ty) => {
        #[tokio::main]
        async fn main() {
            adventofcode2024::run_solution::<$sol>().await;
        }
    };
}
