use std::collections::HashMap;

pub fn run() {
    let input = crate::read_input("day01");

    println!("Solution for day 1a: {}", solve_a(&input));
    println!("Solution for day 1b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> i32 {
    let (mut left, mut right) = get_lists(input);

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .fold(0, |acc, e| acc + (*e.0 - *e.1).abs())
}

fn solve_b(input: &str) -> i32 {
    let (left, right) = get_lists(input);
    let mut map = HashMap::new();

    for number in right {
        let entry = map.entry(number).or_insert(0);
        *entry += 1;
    }

    left.iter()
        .fold(0, |acc, n| acc + *n * map.get(n).copied().unwrap_or(0))
}

fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .flat_map(|line| line.split_ascii_whitespace())
        .map(|n| n.parse::<i32>().unwrap())
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    (
        left.into_iter().map(|(_, n)| n).collect(),
        right.into_iter().map(|(_, n)| n).collect(),
    )
}
