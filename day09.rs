use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("part 1: {}", solution1());
    println!("part 2: {}", solution2());
}

struct Point {
    x: i64,
    y: i64,
}

fn solution1() -> i64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut points: Vec<Point> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(',');
        let x: i64 = parts.next().unwrap().trim().parse().unwrap();
        let y: i64 = parts.next().unwrap().trim().parse().unwrap();
        points.push(Point { x, y });
    }

    let mut answer = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dx = (points[i].x - points[j].x).abs() + 1;
            let dy = (points[i].y - points[j].y).abs() + 1;
            let area = dx * dy;

            if area > answer {
                // println!(
                //     "New rectangle between ({},{}) and ({},{}) with area {}",
                //     points[i].x, points[i].y, points[j].x, points[j].y, area
                // );
                answer = area;
            }
        }
    }

    answer
}

fn solution2() -> i64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut points: Vec<Point> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(',');
        let x: i64 = parts.next().unwrap().parse().unwrap();
        let y: i64 = parts.next().unwrap().parse().unwrap();
        points.push(Point { x, y });
    }

    let mut answer = 0;
    let n = points.len();

    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = &points[i];
            let p2 = &points[j];

            let min_x = min(p1.x, p2.x);
            let max_x = max(p1.x, p2.x);
            let min_y = min(p1.y, p2.y);
            let max_y = max(p1.y, p2.y);

            let dx = (max_x - min_x).abs() + 1;
            let dy = (max_y - min_y).abs() + 1;
            let area = dx * dy;

            if area > answer {
                if is_valid_rectangle(min_x, max_x, min_y, max_y, &points) {
                    answer = area;
                }
            }
        }
    }

    answer
}

fn is_valid_rectangle(
    r_min_x: i64,
    r_max_x: i64,
    r_min_y: i64,
    r_max_y: i64,
    poly: &Vec<Point>,
) -> bool {
    let n = poly.len();

    for k in 0..n {
        let p_a = &poly[k];
        let p_b = &poly[(k + 1) % n];

        if p_a.x == p_b.x {
            let edge_x = p_a.x;
            let edge_min_y = min(p_a.y, p_b.y);
            let edge_max_y = max(p_a.y, p_b.y);

            if edge_x > r_min_x && edge_x < r_max_x {
                if max(edge_min_y, r_min_y) < min(edge_max_y, r_max_y) {
                    return false;
                }
            }
        } else if p_a.y == p_b.y {
            let edge_y = p_a.y;
            let edge_min_x = min(p_a.x, p_b.x);
            let edge_max_x = max(p_a.x, p_b.x);

            if edge_y > r_min_y && edge_y < r_max_y {
                if max(edge_min_x, r_min_x) < min(edge_max_x, r_max_x) {
                    return false;
                }
            }
        }
    }

    let center_x = (r_min_x as f64 + r_max_x as f64) / 2.0;
    let center_y = (r_min_y as f64 + r_max_y as f64) / 2.0;
    let mut inside = false;

    for k in 0..n {
        let p_a = &poly[k];
        let p_b = &poly[(k + 1) % n];

        if (p_a.y > p_b.y) != (p_b.y > p_a.y) {
            let edge_min_y = min(p_a.y, p_b.y) as f64;
            let edge_max_y = max(p_a.y, p_b.y) as f64;
            let edge_x = p_a.x as f64;

            if center_y > edge_min_y && center_y < edge_max_y {
                if edge_x > center_x {
                    inside = !inside;
                }
            }
        }
    }

    inside
}
