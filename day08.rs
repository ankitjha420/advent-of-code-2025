use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

struct Connection {
    u: usize,
    v: usize,
    dist_sq: i64,
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<i64>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let p = self.parent[i];
            self.parent[i] = self.find(p);
            self.parent[i]
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
        }
    }
}

fn main() {
    println!("part 1: {}", solution1());
    println!("part 2: {}", solution2());
}

fn solution1() -> i64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut points: Vec<Point> = Vec::new();
    let mut answer = 1;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        let coords: Vec<i64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
        points.push(Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }

    let mut connections: Vec<Connection> = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let dist_sq = (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2);
            connections.push(Connection {
                u: i,
                v: j,
                dist_sq,
            })
        }
    }

    connections.sort_by(|a, b| a.dist_sq.cmp(&b.dist_sq));
    let mut dsu = DSU::new(points.len());
    let limit = 1000.min(connections.len());
    for conn in connections.iter().take(limit) {
        dsu.union(conn.u, conn.v);
    }

    let mut circuit_sizes: Vec<i64> = Vec::new();
    for i in 0..points.len() {
        if i == dsu.parent[i] {
            circuit_sizes.push(dsu.size[i]);
        }
    }
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    for i in 0..3 {
        if i < circuit_sizes.len() {
            answer *= circuit_sizes[i];
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
        if line.trim().is_empty() {
            continue;
        }

        let coords: Vec<i64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
        points.push(Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }

    let mut connections: Vec<Connection> = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let dist_sq = (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2);
            connections.push(Connection {
                u: i,
                v: j,
                dist_sq,
            })
        }
    }

    connections.sort_by(|a, b| a.dist_sq.cmp(&b.dist_sq));
    let mut dsu = DSU::new(points.len());
    let mut distinct_circuits = points.len();

    for conn in connections {
        if dsu.find(conn.u) != dsu.find(conn.v) {
            dsu.union(conn.u, conn.v);
            distinct_circuits -= 1;
            if distinct_circuits == 1 {
                let x1 = points[conn.u].x;
                let x2 = points[conn.v].x;
                return x1 * x2;
            }
        }
    }

    0
}
