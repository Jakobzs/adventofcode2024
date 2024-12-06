use std::collections::{HashMap, HashSet};

use adventofcode2024::AocSolution;

pub struct Solution;

impl AocSolution for Solution {
    const DAY: u8 = 6;

    fn new() -> Self {
        Self
    }

    fn part1<T: AsRef<str>>(&self, input: T) -> u64 {
        let mut map = input
            .as_ref()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let rows = map.len();
        let cols = map[0].len();

        // Directions: (dy, dx), order: up, right, down, left
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut direction = 0; // Start facing up

        // Find the starting position of the guard
        let mut guard_pos = (0, 0);
        for (i, row) in map.iter().enumerate() {
            if let Some(j) = row
                .iter()
                .position(|&ch| ch == '^' || ch == '>' || ch == 'v' || ch == '<')
            {
                guard_pos = (i, j);
                direction = match map[i][j] {
                    '^' => 0,
                    '>' => 1,
                    'v' => 2,
                    '<' => 3,
                    _ => unreachable!(),
                };
                map[i][j] = '.'; // Clear the starting position
                break;
            }
        }

        let mut visited = HashSet::new();
        visited.insert(guard_pos);

        loop {
            let (y, x) = guard_pos;
            let (dy, dx) = directions[direction];
            let new_pos = (y as isize + dy, x as isize + dx);

            // Check if new position is out of bounds
            if new_pos.0 < 0
                || new_pos.0 >= rows as isize
                || new_pos.1 < 0
                || new_pos.1 >= cols as isize
            {
                break;
            }

            let (ny, nx) = (new_pos.0 as usize, new_pos.1 as usize);
            if map[ny][nx] == '#' {
                // Obstacle: Turn right
                direction = (direction + 1) % 4;
            } else {
                // Move forward
                guard_pos = (ny, nx);
                visited.insert(guard_pos);
            }
        }

        visited.len() as u64
    }

    fn part2<T: AsRef<str>>(&self, input: T) -> u64 {
        0
    }
}

adventofcode2024::run!(Solution);
