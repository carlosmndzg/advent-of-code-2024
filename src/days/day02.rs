pub fn run() {
    let input = crate::read_input("day02");

    println!("Solution for day 2a: {}", solve_a(&input));
    println!("Solution for day 2b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> i32 {
    input
        .lines()
        .map(convert_line_to_report)
        .filter(is_report_safe)
        .count() as i32
}

fn solve_b(input: &str) -> i32 {
    input
        .lines()
        .map(convert_line_to_report)
        .filter(is_report_safe_with_dampener)
        .count() as i32
}

fn convert_line_to_report(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let increasing = report[report.len() - 1] - report[0] >= 0;

    (&report[1..])
        .iter()
        .enumerate()
        .all(|(previous_index, value)| are_valid_levels(report[previous_index], *value, increasing))
}

fn are_valid_levels(level_one: i32, level_two: i32, increasing: bool) -> bool {
    let difference = level_two - level_one;

    return if increasing {
        difference <= 3 && difference >= 1
    } else {
        difference >= -3 && difference <= -1
    };
}

fn is_report_safe_with_dampener(report: &Vec<i32>) -> bool {
    if is_report_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let subset: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, &n)| n)
            .collect();

        if is_report_safe(&subset) {
            return true;
        }
    }

    false
}
