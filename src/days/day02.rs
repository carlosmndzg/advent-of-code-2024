pub fn run() {
    let input = crate::read_input("day02");

    println!("Solution for day 2a: {}", solve_a(&input));
}

fn solve_a(input: &str) -> i32 {
    input
        .lines()
        .map(convert_line_to_report)
        .filter(is_report_safe)
        .count() as i32
}

fn convert_line_to_report(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut iterator = (&report[1..]).iter().enumerate();
    let increasing = report[report.len() - 1] - report[0] >= 0;

    if increasing {
        iterator.all(|(previous_index, value)| {
            let difference = value - report[previous_index];

            difference <= 3 && difference >= 1
        })
    } else {
        iterator.all(|(previous_index, value)| {
            let difference = report[previous_index] - value;

            difference <= 3 && difference >= 1
        })
    }
}
