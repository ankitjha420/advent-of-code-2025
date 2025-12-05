use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("part 1: {}", solution1());
    println!("part 2: {}", solution2());
}

fn solution1() -> i64 {
    let mut answer = 0;
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut is_reading_ranges = true;

    for line in reader.lines() {
        let line_content = line.unwrap();
        let trimmed_line = line_content.trim();

        if trimmed_line.is_empty() {
            is_reading_ranges = false;
            continue;
        }

        if is_reading_ranges {
            if let Some((start_str, end_str)) = trimmed_line.split_once('-') {
                let start: i64 = start_str.parse().unwrap();
                let end: i64 = end_str.parse().unwrap();
                ranges.push((start, end));
            }
        } else {
            let num: i64 = trimmed_line.parse().unwrap();
            let is_valid = ranges
                .iter()
                .any(|&(start, end)| num >= start && num <= end);
            if is_valid {
                answer += 1;
            }
        }
    }

    answer
}

fn solution2() -> i64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for line in reader.lines() {
        let line_content = line.unwrap();
        let trimmed = line_content.trim();

        if trimmed.is_empty() {
            break;
        }

        if let Some((start_str, end_str)) = trimmed.split_once('-') {
            let start: i64 = start_str.parse().unwrap();
            let end: i64 = end_str.parse().unwrap();
            ranges.push((start, end));
        }
    }

    if ranges.is_empty() {
        return 0;
    }
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut total_count: i64 = 0;
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for &(next_start, next_end) in ranges.iter().skip(1) {
        if next_start <= current_end {
            current_end = cmp::max(current_end, next_end);
        } else {
            total_count += current_end - current_start + 1;
            current_start = next_start;
            current_end = next_end;
        }
    }

    total_count += current_end - current_start + 1;

    total_count
}
