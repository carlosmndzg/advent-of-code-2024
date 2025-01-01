use regex::Regex;

pub fn run() {
    let input = crate::read_input("day03");

    println!("Solution for day 3a: {}", solve_a(&input));
    println!("Solution for day 3b: {}", solve_b(&input));
}

const SHAPE: &str = "mul(x,x)";
const FIRST_NUMBER_PLACEHOLDER: usize = 4;
const MIN_LENGTH_NUMBER: usize = 1;
const MAX_LENGTH_NUMBER: usize = 3;

// Implementation of the exercise without using regular expressions
fn solve_a(input: &str) -> i32 {
    let shape: Vec<char> = SHAPE.chars().collect();
    let input: Vec<char> = input.chars().collect();
    let mut sum = 0;
    let mut i = 0;
    let mut position = 0;
    let mut first_number = 0;
    let mut second_number = 0;

    while i < input.len() {
        let c = input[i];

        if shape[position] == 'x' {
            let mut j = 0;

            while (i + j) < input.len() && input[i + j].is_ascii_digit() {
                j += 1;
            }

            if j < MIN_LENGTH_NUMBER || j > MAX_LENGTH_NUMBER {
                position = 0
            } else {
                let number = (&input[i..(i + j)])
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();

                if position == FIRST_NUMBER_PLACEHOLDER {
                    first_number = number;
                } else {
                    second_number = number;
                }

                position += 1;
            }

            i += j;
        } else if c == shape[position] {
            if position == shape.len() - 1 {
                sum += first_number * second_number;
                position = 0
            } else {
                position += 1;
            }

            i += 1;
        } else {
            position = if c == 'm' { 1 } else { 0 };
            i += 1;
        }
    }

    sum
}

// Implementation of the exercise with regular expression because it is more complex.
fn solve_b(input: &str) -> i32 {
    let mut mul_values = get_mul_values(input).into_iter().peekable();
    let mut do_values = get_do_values(input).into_iter().peekable();
    let mut dont_values = get_dont_values(input).into_iter().peekable();

    let mut mul_enabled = true;
    let mut result = 0;

    while mul_values.peek().is_some() {
        let index_mul = mul_values.peek().unwrap().0;
        let index_do = do_values.peek().copied().unwrap_or(input.len());
        let index_dont = dont_values.peek().copied().unwrap_or(input.len());

        if index_mul < index_do && index_mul < index_dont {
            let mul_value = mul_values.next().unwrap().1;

            if mul_enabled {
                result += mul_value;
            }
        } else {
            if index_do < index_dont && index_do != input.len() {
                mul_enabled = true;
                do_values.next();
            } else if index_dont < index_do && index_dont != input.len() {
                mul_enabled = false;
                dont_values.next();
            }
        }
    }

    result
}

fn get_mul_values(input: &str) -> Vec<(usize, i32)> {
    let mul_regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    mul_regex
        .captures_iter(input)
        .map(|c| {
            (
                c.get(0).unwrap().start(),
                c.get(1).unwrap().as_str().parse::<i32>().unwrap()
                    * c.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn get_do_values(input: &str) -> Vec<usize> {
    let do_regex = Regex::new(r"do\(\)").unwrap();

    do_regex.find_iter(input).map(|c| c.start()).collect()
}

fn get_dont_values(input: &str) -> Vec<usize> {
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    dont_regex.find_iter(input).map(|c| c.start()).collect()
}
