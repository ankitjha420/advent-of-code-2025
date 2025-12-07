use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("part 1: {}", solution1());
    println!("part 2: {}", solution2());
}

fn solution1() -> i64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut answer = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();

        grid.push(chars);
    }

    let start_position: usize = grid[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("S not found in first row");

    let mut beam_positions: Vec<usize> = Vec::new();
    beam_positions.push(start_position);

    for i in 1..grid.len() {
        let row = &grid[i];
        let mut row_beam_positions: Vec<usize> = Vec::new();

        for pos in &beam_positions {
            let current_char = row[*pos];
            if current_char == '^' {
                answer += 1;
                if *pos > 0 {
                    row_beam_positions.push(*pos - 1);
                }
                if *pos + 1 < row.len() {
                    row_beam_positions.push(*pos + 1);
                }
            } else {
                row_beam_positions.push(*pos);
            }
        }

        row_beam_positions.sort_unstable();
        row_beam_positions.dedup();
        beam_positions = row_beam_positions;
    }

    // beam_positions.len() as i64
    answer
}

fn solution2() -> i64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    let start_position: usize = grid[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("S not found in first row");

    let mut timeline_counts: HashMap<usize, u64> = HashMap::new();
    timeline_counts.insert(start_position, 1);

    for r in 1..grid.len() {
        let row = &grid[r];
        let mut next_row_counts: HashMap<usize, u64> = HashMap::new();

        for (&pos, &count) in &timeline_counts {
            let current_char = row[pos];
            if current_char == '^' {
                if pos > 0 {
                    *next_row_counts.entry(pos - 1).or_insert(0) += count;
                }
                if pos + 1 < row.len() {
                    *next_row_counts.entry(pos + 1).or_insert(0) += count;
                }
            } else {
                *next_row_counts.entry(pos).or_insert(0) += count;
            }
        }

        timeline_counts = next_row_counts;
    }

    timeline_counts.values().sum::<u64>() as i64
}
