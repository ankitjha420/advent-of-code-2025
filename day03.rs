use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("part 1: {}", solution1());
    println!("part 2: {}", solution2());
}

fn solution1() -> i32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let digits: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        let (x, y) = two_greatest(&digits).unwrap();
        println!("two cells: {x}{y}");
        total_sum += (x * 10) + y;
    }

    total_sum
}

fn solution2() -> i64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let digits: Vec<i64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();

        let num: i64 = twelve_greatest(&digits).unwrap();
        println!("number created: {num}");
        total_sum += num;
    }

    total_sum
}

fn two_greatest(nums: &[i32]) -> Option<(i32, i32)> {
    if nums.len() < 2 {
        return None;
    }
    let max_first = *nums[..nums.len() - 1].iter().max()?;
    let idx = nums.iter().position(|&x| x == max_first)?;

    let max_second = *nums[idx + 1..].iter().max()?;
    Some((max_first, max_second))
}

fn twelve_greatest(nums: &[i64]) -> Option<i64> {
    let k = 12;
    let n = nums.len();

    if n < k {
        return None;
    }

    let to_drop = n - k;
    let mut drop = to_drop as i64;
    let mut stack: Vec<i64> = Vec::with_capacity(k);

    for &num in nums {
        while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < num {
            stack.pop();
            drop -= 1;
        }

        stack.push(num);
    }

    stack.truncate(k);
    let result = stack.iter().fold(0_i64, |acc, &d| acc * 10 + d);

    Some(result)
}
