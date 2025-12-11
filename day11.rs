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
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            continue;
        }

        if let Some((src, dests)) = line.split_once(':') {
            let src = src.trim().to_string();
            let outputs = dests
                .trim()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            graph.insert(src, outputs);
        }
    }

    let mut memo: HashMap<String, i64> = HashMap::new();
    count_paths("you", "out", &graph, &mut memo)
}

fn count_paths(
    current_node: &str,
    target_node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if current_node == target_node {
        return 1;
    }

    if let Some(&count) = memo.get(current_node) {
        return count;
    }

    let mut total_paths = 0;
    if let Some(neighbors) = graph.get(current_node) {
        for neighbor in neighbors {
            total_paths += count_paths(neighbor, target_node, graph, memo);
        }
    }

    memo.insert(current_node.to_string(), total_paths);
    total_paths
}

fn solution2() -> i64 {
    let file = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            continue;
        }

        if let Some((src, dests)) = line.split_once(':') {
            let src = src.trim().to_string();
            let outputs = dests
                .trim()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            graph.insert(src, outputs);
        }
    }

    // svr -> dac -> fft -> out
    let p1 = count_paths("svr", "dac", &graph, &mut HashMap::new());
    let p2 = count_paths("dac", "fft", &graph, &mut HashMap::new());
    let p3 = count_paths("fft", "out", &graph, &mut HashMap::new());
    let route_a = p1 * p2 * p3;
    println!("{p1} {p2} {p3}");

    // svr -> fft -> dac -> out
    let p4 = count_paths("svr", "fft", &graph, &mut HashMap::new());
    let p5 = count_paths("fft", "dac", &graph, &mut HashMap::new());
    let p6 = count_paths("dac", "out", &graph, &mut HashMap::new());
    let route_b = p4 * p5 * p6;
    println!("{p4} {p5} {p6}");

    route_b + route_a
}

/*fn count_paths_between(
    current_node: &str,
    target_node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if current_node == target_node {
        return 1;
    }

    if let Some(&count) = memo.get(current_node) {
        return count;
    }

    let mut total_paths = 0;
    if let Some(neighbors) = graph.get(current_node) {
        for neighbor in neighbors {
            total_paths += count_paths_between(neighbor, target_node, graph, memo);
        }
    }

    memo.insert(current_node.to_string(), total_paths);
    total_paths
}*/
