struct Equation {
    test: u128,
    operands: Vec<u128>,
}

impl Equation {
    fn new(test: u128, operands: Vec<u128>) -> Self {
        Self { test, operands }
    }

    fn is_possible_to_solve(&self) -> bool {
        self.backtracking(self.operands[0], 1)
    }

    fn is_possible_to_solve_2(&self) -> bool {
        self.backtracking_2(self.operands[0], 1)
    }

    fn backtracking(&self, value: u128, index: usize) -> bool {
        if index == self.operands.len() {
            return value == self.test;
        }

        let operand = self.operands[index];

        self.backtracking(value + operand, index + 1)
            || self.backtracking(value * operand, index + 1)
    }

    fn backtracking_2(&self, value: u128, index: usize) -> bool {
        if index == self.operands.len() {
            return value == self.test;
        }

        let operand = self.operands[index];
        let power_of_ten = 10u128.pow(count_digits(operand));

        self.backtracking_2(value + operand, index + 1)
            || self.backtracking_2(value * operand, index + 1)
            || self.backtracking_2(value * power_of_ten + operand, index + 1)
    }
}

pub fn run() {
    let input = crate::read_input("day07");

    println!("Solution for day 7a: {}", solve_a(&input));
    println!("Solution for day 7b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> u128 {
    let equations = parse_equations(input);

    equations
        .iter()
        .filter(|equation| equation.is_possible_to_solve())
        .map(|equation| equation.test)
        .sum()
}

fn solve_b(input: &str) -> u128 {
    let equations = parse_equations(input);

    equations
        .iter()
        .filter(|equation| equation.is_possible_to_solve_2())
        .map(|equation| equation.test)
        .sum()
}

fn parse_equations(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let test = parts.next().unwrap().parse().unwrap();

            let operands = parts
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|part| part.trim().parse().unwrap())
                .collect();

            Equation::new(test, operands)
        })
        .collect()
}

fn count_digits(mut number: u128) -> u32 {
    let mut count = 0;

    while number > 0 {
        number /= 10;
        count += 1;
    }

    count
}
