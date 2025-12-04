use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("part 1: {}", solution1());
    println!("part 2: {}", solution2());
}

fn solution1() -> i32 {
    let mut answer = 0;
    let file = File::open("input.txt").expect("Could not open test.txt");
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    let rows = grid.len();
    if rows == 0 { return 0; }
    let cols = grid[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '.' {
                continue;
            }
            let mut neighbor_count = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }

                    let new_r = r as isize + dy;
                    let new_c = c as isize + dx;

                    if new_r >= 0 && new_r < rows as isize &&
                        new_c >= 0 && new_c < cols as isize {
                        if grid[new_r as usize][new_c as usize] == '@' {
                            neighbor_count += 1;
                        }
                    }
                }
            }

            if neighbor_count < 4 {
                answer += 1;
            }
        }
    }

    answer
}

fn solution2() -> i32 {
    let mut answer = 0;
    let file = File::open("input.txt").expect("Could not open input.txt");
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        grid.push(line.chars().collect());
    }

    let rows = grid.len();
    if rows == 0 { return 0; }
    let cols = grid[0].len();

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '.' {
                    continue;
                }
                let mut neighbor_count = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 { continue; }
                        let new_r = r as isize + dy;
                        let new_c = c as isize + dx;

                        if new_r >= 0 && new_r < rows as isize &&
                            new_c >= 0 && new_c < cols as isize {
                            if grid[new_r as usize][new_c as usize] == '@' {
                                neighbor_count += 1;
                            }
                        }
                    }
                }

                if neighbor_count < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }
        answer += to_remove.len() as i32;

        println!("Round removed: {}", to_remove.len());
        for (r, c) in to_remove {
            grid[r][c] = '.';
        }
    }

    answer
}
