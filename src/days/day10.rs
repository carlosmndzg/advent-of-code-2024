use std::collections::HashSet;

const TRAILHEAD: char = '0';
const FINISHED: char = '9';

pub fn run() {
    let input = crate::read_input("day10");

    println!("Solution for day 10a: {}", solve_a(&input));
    println!("Solution for day 10b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> i32 {
    let map = parse_map(input);
    let trailheads = find_trailheads(&map);

    trailheads
        .iter()
        .map(|trailhead| compute_score(&map, trailhead))
        .sum()
}

fn solve_b(input: &str) -> i32 {
    let map = parse_map(input);
    let trailheads = find_trailheads(&map);

    trailheads
        .iter()
        .map(|trailhead| compute_rating(&map, trailhead))
        .sum()
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_trailheads(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();

    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == TRAILHEAD {
                trailheads.push((i, j));
            }
        }
    }

    trailheads
}

fn compute_score(map: &Vec<Vec<char>>, trailhead: &(usize, usize)) -> i32 {
    let mut finished = HashSet::new();

    backtracking(map, trailhead, &mut finished);

    finished.len() as i32
}

fn backtracking(
    map: &Vec<Vec<char>>,
    current: &(usize, usize),
    finished: &mut HashSet<(usize, usize)>,
) {
    if map[current.0][current.1] == FINISHED {
        finished.insert(*current);
        return;
    }

    for neighbor in neighbors(map, current) {
        backtracking(map, &neighbor, finished);
    }
}

fn neighbors(map: &Vec<Vec<char>>, current: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let next_value = (map[current.0][current.1] as u8 + 1) as char;

    if current.0 > 0 && map[current.0 - 1][current.1] == next_value {
        neighbors.push((current.0 - 1, current.1));
    }

    if current.0 < map.len() - 1 && map[current.0 + 1][current.1] == next_value {
        neighbors.push((current.0 + 1, current.1));
    }

    if current.1 > 0 && map[current.0][current.1 - 1] == next_value {
        neighbors.push((current.0, current.1 - 1));
    }

    if current.1 < map[0].len() - 1 && map[current.0][current.1 + 1] == next_value {
        neighbors.push((current.0, current.1 + 1));
    }

    neighbors
}

fn compute_rating(map: &Vec<Vec<char>>, trailhead: &(usize, usize)) -> i32 {
    backtracking_rating(map, trailhead)
}

fn backtracking_rating(map: &Vec<Vec<char>>, current: &(usize, usize)) -> i32 {
    if map[current.0][current.1] == FINISHED {
        return 1;
    }

    let mut rating = 0;

    for neighbor in neighbors(map, current) {
        rating += backtracking_rating(map, &neighbor);
    }

    rating
}
