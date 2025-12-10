use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,      // Moves so far (g score)
    heuristic: usize, // Estimated moves remaining (h score)
    values: Vec<u64>, // Current counters
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want the lowest (cost + heuristic) to pop first.
        // BinaryHeap is a Max-Heap, so we invert the comparison.
        let my_total = self.cost + self.heuristic;
        let other_total = other.cost + other.heuristic;

        other_total
            .cmp(&my_total)
            .then_with(|| other.cost.cmp(&self.cost)) // Tie-breaker
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    println!("part 1: {}", solution1());
    println!("part 2: {}", solution2());
}

fn solution1() -> i64 {
    let file = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    let mut total_presses = 0;

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            continue;
        }

        let start_bracket = line.find('[').unwrap();
        let end_bracket = line.find(']').unwrap();
        let target_str = &line[start_bracket + 1..end_bracket];

        let mut target_mask: u64 = 0;

        for (i, c) in target_str.chars().enumerate() {
            if c == '#' {
                target_mask |= 1 << i;
            }
        }

        let mut buttons: Vec<u64> = Vec::new();
        let mut string_rest = &line[end_bracket..];

        while let Some(open) = string_rest.find('(') {
            if let Some(close) = string_rest.find(')') {
                let content = &string_rest[open + 1..close];
                let mut button_mask: u64 = 0;

                for part in content.split(',') {
                    if let Ok(idx) = part.trim().parse::<usize>() {
                        button_mask |= 1 << idx;
                    }
                }
                buttons.push(button_mask);
                string_rest = &string_rest[close + 1..];
            } else {
                break;
            }
        }

        if let Some(presses) = bfs_min_presses(target_mask, &buttons) {
            println!("Target: {:b} solved in {} presses", target_mask, presses);
            total_presses += presses as i64;
        } else {
            println!("Target: {:b} is impossible", target_mask);
        }
    }

    total_presses
}

fn bfs_min_presses(target: u64, buttons: &[u64]) -> Option<usize> {
    let mut queue: VecDeque<(u64, usize)> = VecDeque::new();
    let mut visited: HashSet<u64> = HashSet::new();

    queue.push_back((0, 0));
    visited.insert(0);

    while let Some((current_state, steps)) = queue.pop_front() {
        if current_state == target {
            return Some(steps);
        }

        for &button_mask in buttons {
            let next_state = current_state ^ button_mask;

            if !visited.contains(&next_state) {
                visited.insert(next_state);
                queue.push_back((next_state, steps + 1));
            }
        }
    }

    None
}

fn solution2() -> i64 {
    let file = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    let mut total_presses = 0;

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            continue;
        }

        let curly_start = line.find('{').expect("No target found");
        let curly_end = line.find('}').expect("No target found");
        let target_str = &line[curly_start + 1..curly_end];

        let target: Vec<u64> = target_str
            .split(',')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect();

        let num_counters = target.len();

        let schematic_part = &line[0..curly_start];
        let mut buttons: Vec<Vec<u64>> = Vec::new();

        let mut str_rest = schematic_part;
        while let Some(open) = str_rest.find('(') {
            if let Some(close) = str_rest.find(')') {
                let content = &str_rest[open + 1..close];

                let mut btn_vec = vec![0u64; num_counters];
                for part in content.split(',') {
                    if let Ok(idx) = part.trim().parse::<usize>() {
                        if idx < num_counters {
                            btn_vec[idx] = 1;
                        }
                    }
                }
                buttons.push(btn_vec);

                str_rest = &str_rest[close + 1..];
            } else {
                break;
            }
        }

        if let Some(presses) = astar_counters(&target, &buttons) {
            println!("Target {:?} solved in {} presses", target, presses);
            total_presses += presses as i64;
        } else {
            println!("Target {:?} is impossible", target);
        }
    }

    total_presses
}

fn heuristic(current: &[u64], target: &[u64]) -> usize {
    let mut max_diff = 0;
    for (c, t) in current.iter().zip(target.iter()) {
        if t > c {
            let diff = (t - c) as usize;
            if diff > max_diff {
                max_diff = diff;
            }
        }
    }
    max_diff
}

fn astar_counters(target: &[u64], buttons: &[Vec<u64>]) -> Option<usize> {
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    let start_values = vec![0u64; target.len()];
    let start_h = heuristic(&start_values, target);

    pq.push(State {
        cost: 0,
        heuristic: start_h,
        values: start_values.clone(),
    });
    visited.insert(start_values);

    while let Some(State { cost, values, .. }) = pq.pop() {
        if &values == target {
            return Some(cost);
        }

        if values.iter().zip(target.iter()).any(|(v, t)| v > t) {
            continue;
        }

        for btn in buttons {
            let mut next_values = values.clone();
            for i in 0..target.len() {
                next_values[i] += btn[i];
            }

            if !visited.contains(&next_values) {
                if next_values.iter().zip(target.iter()).any(|(v, t)| v > t) {
                    continue;
                }

                if &next_values == target {
                    return Some(cost + 1);
                }

                let h = heuristic(&next_values, target);
                visited.insert(next_values.clone());

                pq.push(State {
                    cost: cost + 1,
                    heuristic: h,
                    values: next_values,
                });
            }
        }
    }

    None
}
