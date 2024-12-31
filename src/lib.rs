use std::fs;

pub mod days;

fn read_input(day: &str) -> String {
    fs::read_to_string(format!("./inputs/{day}.txt")).expect("Failed to read input file")
}
