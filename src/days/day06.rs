use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn run() {
    let input = crate::read_input("day06");

    println!("Solution for day 6a: {}", solve_a(&input));
    println!("Solution for day 6b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> i32 {
    let map = create_map(input);
    let initial_position = find_initial_position(&map);
    let initial_direction = Direction::Up;
    let mut unique_positions_without_direction = HashSet::new();

    find_positions_visited(&map, initial_position, initial_direction)
        .iter()
        .for_each(|&(i, j, _)| {
            unique_positions_without_direction.insert((i, j));
        });

    unique_positions_without_direction.len() as i32
}

fn solve_b(input: &str) -> i32 {
    let mut map = create_map(input);
    let initial_position = find_initial_position(&map);
    let initial_direction = Direction::Up;
    let positions = find_positions_visited(&map, initial_position, initial_direction.clone());
    let mut positions_of_loop = HashSet::new();

    for (i, j, direction) in positions.iter() {
        let position = (*i, *j);

        if check_out_of_bounds_with_next_position(&map, &position, direction) {
            continue;
        }

        let next_position = get_next_position(&position, direction);

        if get_char_at_position(&map, &next_position) == '#' {
            continue;
        }

        map[next_position.0][next_position.1] = '#';

        if check_for_loop(&map, initial_position, initial_direction.clone()) {
            positions_of_loop.insert(next_position);
        }

        map[next_position.0][next_position.1] = '.';
    }

    positions_of_loop.len() as i32
}

fn find_positions_visited(
    map: &Vec<Vec<char>>,
    initial_position: (usize, usize),
    initial_direction: Direction,
) -> HashSet<(usize, usize, Direction)> {
    let mut position = initial_position;
    let mut direction = initial_direction;
    let mut positions = HashSet::new();

    loop {
        positions.insert((position.0, position.1, direction.clone()));

        if check_out_of_bounds_with_next_position(&map, &position, &direction) {
            break;
        }

        let next_position = get_next_position(&position, &direction);

        if get_char_at_position(&map, &next_position) == '#' {
            direction = get_next_direction(&direction);
        } else {
            position = next_position;
        }
    }

    positions
}

fn create_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_initial_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for (j, &c) in map[i].iter().enumerate() {
            if c == '^' {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn check_out_of_bounds_with_next_position(
    map: &Vec<Vec<char>>,
    &(i, j): &(usize, usize),
    direction: &Direction,
) -> bool {
    match direction {
        Direction::Up => i == 0,
        Direction::Down => i == map.len() - 1,
        Direction::Left => j == 0,
        Direction::Right => j == map[i].len() - 1,
    }
}

fn get_char_at_position(map: &Vec<Vec<char>>, &(i, j): &(usize, usize)) -> char {
    map[i][j]
}

fn get_next_position(&(i, j): &(usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (i - 1, j),
        Direction::Down => (i + 1, j),
        Direction::Left => (i, j - 1),
        Direction::Right => (i, j + 1),
    }
}

fn get_next_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn check_for_loop(
    map: &Vec<Vec<char>>,
    initial_position: (usize, usize),
    initial_direction: Direction,
) -> bool {
    let mut position = initial_position;
    let mut direction = initial_direction;
    let mut positions = HashSet::new();

    loop {
        if positions.contains(&(position.0, position.1, direction.clone())) {
            return true;
        } else {
            positions.insert((position.0, position.1, direction.clone()));
        }

        if check_out_of_bounds_with_next_position(&map, &position, &direction) {
            return false;
        }

        let next_position = get_next_position(&position, &direction);

        if get_char_at_position(&map, &next_position) == '#' {
            direction = get_next_direction(&direction);
        } else {
            position = next_position;
        }
    }
}
