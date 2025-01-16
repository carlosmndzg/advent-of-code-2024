use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = crate::read_input("day08");

    println!("Solution for day 8a: {}", solve_a(&input));
    println!("Solution for day 8b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> i32 {
    let map = parse_map(input);
    let frequencies = find_frequencies(&map);
    let mut antinodes = HashSet::new();

    for frequency in frequencies.keys() {
        let nodes = frequencies.get(frequency).unwrap();

        for i in 0..nodes.len() - 1 {
            for j in i + 1..nodes.len() {
                let node1 = nodes[i];
                let node2 = nodes[j];
                let (dx, dy) = get_difference(node1, node2);

                let first_antinode = (node1.0 as i32 - dx, node1.1 as i32 - dy);
                let second_antinode = (node2.0 as i32 + dx, node2.1 as i32 + dy);

                if is_in_bounds(&map, first_antinode) {
                    antinodes.insert(first_antinode);
                }

                if is_in_bounds(&map, second_antinode) {
                    antinodes.insert(second_antinode);
                }
            }
        }
    }

    antinodes.len() as i32
}

fn solve_b(input: &str) -> i32 {
    let map = parse_map(input);
    let frequencies = find_frequencies(&map);
    let mut antinodes = HashSet::new();

    for frequency in frequencies.keys() {
        let nodes = frequencies.get(frequency).unwrap();

        for i in 0..nodes.len() - 1 {
            for j in i + 1..nodes.len() {
                let node1 = nodes[i];
                let node2 = nodes[j];
                let (dx, dy) = get_difference(node1, node2);

                antinodes.insert((node1.0 as i32, node1.1 as i32));
                antinodes.insert((node2.0 as i32, node2.1 as i32));

                search_for_antinodes(&map, &mut antinodes, node1, (dx, dy), true);
                search_for_antinodes(&map, &mut antinodes, node2, (dx, dy), false);
            }
        }
    }

    antinodes.len() as i32
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_frequencies(map: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut frequencies = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let c = map[i][j];

            if c == '.' {
                continue;
            }

            let entry = frequencies.entry(c).or_insert(Vec::new());
            entry.push((i, j));
        }
    }

    frequencies
}

fn get_difference(node1: (usize, usize), node2: (usize, usize)) -> (i32, i32) {
    let dx = node2.0 as i32 - node1.0 as i32;
    let dy = node2.1 as i32 - node1.1 as i32;

    (dx, dy)
}

fn is_in_bounds(map: &Vec<Vec<char>>, node: (i32, i32)) -> bool {
    let (x, y) = node;

    x >= 0 && x < map.len() as i32 && y >= 0 && y < map[0].len() as i32
}

fn search_for_antinodes(
    map: &Vec<Vec<char>>,
    antinodes: &mut HashSet<(i32, i32)>,
    node: (usize, usize),
    diff: (i32, i32),
    backwards: bool,
) {
    let mut antinode = if backwards {
        (node.0 as i32 - diff.0, node.1 as i32 - diff.1)
    } else {
        (node.0 as i32 + diff.0, node.1 as i32 + diff.1)
    };

    while is_in_bounds(map, antinode) {
        antinodes.insert(antinode);

        antinode = if backwards {
            (antinode.0 - diff.0, antinode.1 - diff.1)
        } else {
            (antinode.0 + diff.0, antinode.1 + diff.1)
        };
    }
}
