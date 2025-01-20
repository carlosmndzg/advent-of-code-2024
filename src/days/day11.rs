use std::collections::HashMap;

pub fn run() {
    let input = crate::read_input("day11");

    println!("Solution for day 11a: {}", solve_a(&input));
    println!("Solution for day 11b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> u128 {
    let mut memory = HashMap::new();
    let stones = parse_stones(input);

    stones
        .into_iter()
        .map(|stone| get_number_of_stones_after_blink(stone, 25, &mut memory))
        .sum()
}

fn solve_b(input: &str) -> u128 {
    let mut memory = HashMap::new();
    let stones = parse_stones(input);

    stones
        .into_iter()
        .map(|stone| get_number_of_stones_after_blink(stone, 75, &mut memory))
        .sum()
}

fn get_number_of_stones_after_blink(
    stone: u128,
    iterations: usize,
    memory: &mut HashMap<(u128, usize), u128>,
) -> u128 {
    if iterations == 0 {
        return 1;
    } else if memory.contains_key(&(stone, iterations)) {
        return memory[&(stone, iterations)];
    } else if stone == 0 {
        let ans = get_number_of_stones_after_blink(1, iterations - 1, memory);
        memory.insert((stone, iterations), ans);
    } else if has_even_number_of_digits(stone) {
        let (left, right) = get_numbers_divided(stone);
        let ans = get_number_of_stones_after_blink(left, iterations - 1, memory)
            + get_number_of_stones_after_blink(right, iterations - 1, memory);

        memory.insert((stone, iterations), ans);
    } else {
        let ans = get_number_of_stones_after_blink(stone * 2024, iterations - 1, memory);
        memory.insert((stone, iterations), ans);
    }

    memory[&(stone, iterations)]
}

fn parse_stones(input: &str) -> Vec<u128> {
    input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_numbers_divided(stone: u128) -> (u128, u128) {
    let number = stone.to_string();
    let first = number[0..number.len() / 2].parse().unwrap();
    let second = number[number.len() / 2..].parse().unwrap();

    (first, second)
}

fn has_even_number_of_digits(stone: u128) -> bool {
    let mut stone = stone;
    let mut digits = 0;

    while stone > 0 {
        stone /= 10;
        digits += 1;
    }

    digits % 2 == 0
}
