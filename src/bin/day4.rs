use adventofcode2024::AocSolution;

pub struct Solution;

impl AocSolution for Solution {
    const DAY: u8 = 4;

    fn new() -> Self {
        Self
    }

    fn part1<T: AsRef<str>>(&self, input: T) -> u64 {
        let grid = input
            .as_ref()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let word = "XMAS";
        let count = count_word_in_grid(&grid, word);

        count as u64
    }

    fn part2<T: AsRef<str>>(&self, input: T) -> u64 {
        let grid = input
            .as_ref()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        find_x_mas(grid) as u64
    }
}

fn count_word_in_grid(grid: &[Vec<char>], word: &str) -> usize {
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Possible directions: (row_offset, col_offset)
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (0, -1),  // Left
        (-1, 0),  // Up
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
    ];

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            // Check for the word in all directions
            for &(row_offset, col_offset) in &directions {
                let mut match_found = true;
                for i in 0..word_len {
                    let new_row = row as isize + i as isize * row_offset;
                    let new_col = col as isize + i as isize * col_offset;

                    if new_row < 0
                        || new_col < 0
                        || new_row >= rows as isize
                        || new_col >= cols as isize
                        || grid[new_row as usize][new_col as usize] != word_chars[i]
                    {
                        match_found = false;
                        break;
                    }
                }
                if match_found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn find_x_mas(grid: Vec<Vec<char>>) -> usize {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut xmas_count: usize = 0;

    const MAS: [char; 3] = ['M', 'A', 'S'];
    const SAM: [char; 3] = ['S', 'A', 'M'];

    for row in 1..num_rows - 1 {
        for col in 1..num_cols - 1 {
            let seq_1 = [
                grid[row - 1][col - 1],
                grid[row][col],
                grid[row + 1][col + 1],
            ];

            let seq_2 = [
                grid[row - 1][col + 1],
                grid[row][col],
                grid[row + 1][col - 1],
            ];

            if (SAM == seq_1 || MAS == seq_1) && (SAM == seq_2 || MAS == seq_2) {
                xmas_count += 1;
            }
        }
    }

    xmas_count
}

adventofcode2024::run!(Solution);
