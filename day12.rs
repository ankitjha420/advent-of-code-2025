use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("part 1: {}", solution1());
}

fn solution1() -> i64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let mut sizes = [0i64; 6];

    for i in 0..6 {
        let _header = lines.next();

        for _ in 0..3 {
            if let Some(line) = lines.next() {
                sizes[i] += line.chars().filter(|&c| c == '#').count() as i64;
            }
        }

        let _empty = lines.next();
    }

    let mut answer = 0;

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 2 {
            continue;
        }

        let dims: Vec<&str> = parts[0].split('x').collect();
        let w: i64 = dims[0].trim().parse().unwrap_or(0);
        let h: i64 = dims[1].trim().parse().unwrap_or(0);
        let region_area = w * h;

        let counts: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        let mut total_present_area = 0;
        for (i, &count) in counts.iter().enumerate() {
            if i < 6 {
                total_present_area += count * sizes[i];
            }
        }

        if total_present_area < region_area {
            answer += 1;
        }
    }

    answer
}
