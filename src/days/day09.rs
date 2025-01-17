pub fn run() {
    let input = crate::read_input("day09");

    println!("Solution for day 9a: {}", solve_a(&input));
    println!("Solution for day 9b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> u128 {
    let mut disk_map = parse_disk_map(input);

    move_file_blocks(&mut disk_map);

    compute_checksum(&disk_map)
}

fn solve_b(input: &str) -> u128 {
    let mut disk_map = parse_disk_map(input);

    move_file_blocks_without_fragmentation(&mut disk_map);

    compute_checksum(&disk_map)
}

fn parse_disk_map(input: &str) -> Vec<String> {
    let mut disk_map = Vec::new();

    for (i, c) in input.chars().enumerate() {
        let str_to_write = if i % 2 == 0 {
            (i / 2).to_string()
        } else {
            ".".to_string()
        };

        for _ in 0..c.to_digit(10).unwrap() {
            disk_map.push(str_to_write.clone());
        }
    }

    disk_map
}

fn move_file_blocks(disk_map: &mut Vec<String>) {
    let mut pos_to_place = 0;
    let mut pos_to_look = disk_map.len() - 1;

    while pos_to_place < pos_to_look {
        if disk_map[pos_to_place] != "." {
            pos_to_place += 1;
        } else if disk_map[pos_to_look] == "." {
            pos_to_look -= 1;
        } else {
            disk_map[pos_to_place] = disk_map[pos_to_look].clone();
            disk_map[pos_to_look] = ".".to_string();
            pos_to_place += 1;
            pos_to_look -= 1;
        }
    }
}

fn compute_checksum(disk_map: &[String]) -> u128 {
    disk_map.iter().enumerate().fold(0, |acc, (pos, block_id)| {
        if block_id == "." {
            acc
        } else {
            acc + (pos as u128) * block_id.parse::<u128>().unwrap()
        }
    })
}

fn move_file_blocks_without_fragmentation(disk_map: &mut Vec<String>) {
    let mut position = disk_map.len() - 1;

    loop {
        let next_block = find_next_block(disk_map, position);

        let next_block = match next_block {
            Some((start, end)) => (start, end),
            None => break,
        };

        let block_size = compute_area_size(next_block);

        let next_free_area = find_next_free_area(disk_map, block_size, position);

        let next_free_area = match next_free_area {
            Some((start, end)) => (start, end),
            None => {
                if next_block.0 == 0 {
                    break;
                }

                position = next_block.0 - 1;
                continue;
            }
        };

        for i in next_free_area.0..(next_free_area.0 + block_size) {
            disk_map[i] = disk_map[next_block.1].clone();
        }

        for i in next_block.0..next_block.1 + 1 {
            disk_map[i] = ".".to_string();
        }

        if next_block.0 == 0 {
            break;
        }

        position = next_block.0 - 1;
    }
}

fn find_next_free_area(disk_map: &[String], size: usize, end: usize) -> Option<(usize, usize)> {
    let mut pos = 0;
    let mut current_size = 0;
    let mut inside_free_area = false;

    while pos < end {
        if disk_map[pos] == "." {
            if !inside_free_area {
                current_size = 0;
                inside_free_area = true;
            }

            current_size += 1;

            if current_size == size {
                return Some((pos - size + 1, pos));
            }
        } else {
            inside_free_area = false;
        }

        pos += 1;
    }

    if pos == end {
        return None;
    }

    let start_free_area = pos;

    while pos < end && disk_map[pos] == "." {
        pos += 1;
    }

    Some((start_free_area, pos - 1))
}

fn find_next_block(disk_map: &[String], start: usize) -> Option<(usize, usize)> {
    let mut pos = start;

    while pos > 0 && disk_map[pos] == "." {
        pos -= 1;
    }

    if pos == 0 {
        return None;
    }

    let end_next_block = pos;

    while pos > 0 && disk_map[pos] == disk_map[end_next_block] {
        pos -= 1;
    }

    Some((pos + 1, end_next_block))
}

fn compute_area_size(area: (usize, usize)) -> usize {
    area.1 - area.0 + 1
}
