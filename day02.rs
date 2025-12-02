use std::fs;

fn main() {
    println!("part 1: {}", solution1());
    println!("part 2: {}", solution2());
}

fn solution1() -> i64 {
    let mut total_sum: i64 = 0;
    let contents = fs::read_to_string("input.txt").expect("failed to open file");

    let pairs = contents.split(',');
    for pair in pairs {
        let (left, right) = pair.split_once('-').expect("invalid range format");

        let left: i64 = left.parse().expect("invalid left number");
        let right: i64 = right.parse().expect("invalid left number");

        total_sum += iterate_range(left, right);
    }

    total_sum
}

fn solution2() -> i64 {
    let mut total_sum: i64 = 0;
    let contents = fs::read_to_string("input.txt").expect("failed to open file");

    let pairs = contents.split(',');
    for pair in pairs {
        let (left, right) = pair.split_once('-').expect("invalid range format");

        let left: i64 = left.parse().expect("invalid left number");
        let right: i64 = right.parse().expect("invalid left number");

        total_sum += iterate_range2(left, right);
    }

    total_sum
}

fn iterate_range(left: i64, right: i64) -> i64 {
    let mut sum = 0;

    for n in left..right + 1 {
        let num_string = format!("{n}");
        if num_string.len() % 2 != 0 {
            continue;
        } else {
            if is_repeated(num_string) {
                sum += n;
            }
        }
    }

    sum
}

fn is_repeated(num: String) -> bool {
    let len = num.len();
    let mid = len / 2;
    let (first, second) = num.split_at(mid);

    first == second
}

fn is_repeated2(num: String) -> bool {
    let len = num.len();
    for block_size in 1..=len / 2 {
        if len % block_size != 0 {
            continue;
        }

        let block = &num[..block_size];
        if block.repeat(len / block_size) == num {
            return true;
        }
    }

    false
}

fn iterate_range2(left: i64, right: i64) -> i64 {
    let mut sum = 0;

    for n in left..right + 1 {
        let num_string = format!("{n}");
        if is_repeated2(num_string) {
            sum += n;
        }
    }

    sum
}
