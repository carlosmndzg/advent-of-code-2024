use regex::Regex;

pub fn run() {
    let input = crate::read_input("day03");

    println!("Solution for day 3a: {}", solve_a(&input));
    println!("Solution for day 3b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> i32 {
    get_mul_values(input).iter().map(|v| v.1).sum()
}

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
