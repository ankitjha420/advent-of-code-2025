use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

fn main() {
    println!("Part 1 answer: {}", solution1());

    let (landings, crossings) = solution2();
    println!("Landings: {landings}, Crossings: {crossings}");
}

fn solution1() -> i32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut curr_position = 50;
    let mut answer = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (direction, num) = line.split_at(1);
        let distance: i32 = num.parse().unwrap();
        let direction = if direction == "R" {
            Direction::Right
        } else {
            Direction::Left
        };

        curr_position = movement2(distance, direction, curr_position);
        if curr_position == 0 {
            answer += 1;
        }
    }

    answer
}

fn solution2() -> (i32, i32) {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut count_landings = 0;
    let mut count_crossings = 0;
    let mut curr_position = 50;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let (dir_str, num_str) = line.split_at(1);
        let distance: i32 = num_str.parse().unwrap();
        let direction = if dir_str == "R" {
            Direction::Right
        } else {
            Direction::Left
        };

        let laps = distance / 100;
        count_crossings += laps;

        let remainder = distance % 100;
        if remainder > 0 {
            match direction {
                Direction::Right => {
                    if curr_position + remainder >= 100 {
                        count_crossings += 1;
                    }
                }
                Direction::Left => {
                    if curr_position != 0 && (curr_position - remainder <= 0) {
                        count_crossings += 1;
                    }
                }
            }
        }

        let new_position = movement2(distance, direction, curr_position);

        if new_position == 0 {
            count_landings += 1;
        }

        curr_position = new_position;
    }

    (count_landings, count_crossings)
}

fn movement2(distance: i32, direction: Direction, curr_position: i32) -> i32 {
    let size = 100;
    let delta = match direction {
        Direction::Left => -distance,
        Direction::Right => distance,
    };
    (curr_position + delta).rem_euclid(size)
}
