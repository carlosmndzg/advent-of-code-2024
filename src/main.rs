use std::env;

use advent_of_code_2024::days;

fn main() {
    let day = env::args().nth(1);
    match day {
        Some(day) => run_day(&day),
        None => {
            eprintln!("Please provide a day (e.g., 01)");
            std::process::exit(1);
        }
    }
}

fn run_day(day: &str) {
    match day {
        "01" => days::day01::run(),
        "02" => days::day02::run(),
        "03" => days::day03::run(),
        _ => println!("Day {} not implemented yet!", day),
    }
}
