use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = crate::read_input("day05");

    println!("Solution for day 5a: {}", solve_a(&input));
    println!("Solution for day 5b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> i32 {
    let mut ans = 0;
    let (rules, updates) = get_input(input);

    for update in updates {
        if is_valid_update(&update, &rules) {
            ans += update[update.len() / 2];
        }
    }

    ans
}

fn solve_b(input: &str) -> i32 {
    let mut ans = 0;
    let (rules, updates) = get_input(input);

    for update in updates {
        if !is_valid_update(&update, &rules) {
            ans += get_middle_after_fixing_incorrect_update(&update, &rules);
        }
    }

    ans
}

fn get_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let input = input.replace('\r', "");
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let rules = get_rules(input[0]);
    let updates = input[1]
        .lines()
        .map(|line| {
            line.trim()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    (rules, updates)
}

fn get_rules(input: &str) -> HashMap<i32, HashSet<i32>> {
    let mut rules = HashMap::new();

    let rules_raw = input
        .lines()
        .map(|line| {
            let line = line.trim().split('|').collect::<Vec<&str>>();

            (
                line[0].parse::<i32>().unwrap(),
                line[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32)>>();

    for line in rules_raw {
        let rule = rules.entry(line.0).or_insert(HashSet::new());
        rule.insert(line.1);
    }

    rules
}

fn is_valid_update(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let numbers_index = get_map_with_number_indexes(update);

    for &number in update {
        for &rule in rules.get(&number).unwrap_or(&HashSet::new()) {
            if let Some(index) = numbers_index.get(&rule) {
                if index < numbers_index.get(&number).unwrap() {
                    return false;
                }
            }
        }
    }

    true
}

fn get_map_with_number_indexes(vec: &[i32]) -> HashMap<i32, usize> {
    let mut map = HashMap::new();

    for (index, number) in vec.iter().enumerate() {
        map.insert(*number, index);
    }

    map
}

/// In case that the update it is incorrect, we return the middle number after fixing it. Otherwise, it returns 0.
fn get_middle_after_fixing_incorrect_update(
    update: &Vec<i32>,
    rules: &HashMap<i32, HashSet<i32>>,
) -> i32 {
    let fixed_update = find_fixed_update(update.clone(), rules);

    fixed_update[fixed_update.len() / 2]
}

fn find_fixed_update(mut update: Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut i = 0;
    let mut number_indexes = get_map_with_number_indexes(&update);

    while i < update.len() {
        let number = update[i];
        let mut exchanged = false;

        for &rule in rules.get(&number).unwrap_or(&HashSet::new()) {
            if let Some(index) = number_indexes.get(&rule) {
                if *index < i {
                    update.swap(i, *index);

                    if is_valid_update(&update[0..=i], rules) {
                        number_indexes = get_map_with_number_indexes(&update);
                        exchanged = true;
                        break;
                    } else {
                        update.swap(i, *index);
                    }
                }
            }
        }

        if !exchanged && !is_valid_update(&update[0..=i], rules) && i != 0 {
            let mut j = i - 1;

            loop {
                update[j + 1] = update[j];
                update[j] = number;

                if is_valid_update(&update[0..=i], rules) {
                    number_indexes = get_map_with_number_indexes(&update);
                    break;
                }

                if j == 0 {
                    break;
                }

                j -= 1;
            }
        }

        if !exchanged {
            i += 1;
        }
    }

    update
}
