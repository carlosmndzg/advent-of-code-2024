const SIZE_XMAS: usize = 4;
const SIZE_MAS: usize = 3;

pub fn run() {
    let input = crate::read_input("day04");

    println!("Solution for day 4a: {}", solve_a(&input));
    println!("Solution for day 4b: {}", solve_b(&input));
}

fn solve_a(input: &str) -> i32 {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    find_xmas_in_rows(&input) + find_xmas_in_columns(&input) + find_xmas_in_diagonals(&input)
}

fn solve_b(input: &str) -> i32 {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    find_mas(&input)
}

fn find_xmas_in_rows(input: &Vec<Vec<char>>) -> i32 {
    let mut ans = 0;

    for i in 0..input.len() {
        for j in 0..(input[i].len() - SIZE_XMAS + 1) {
            let word = (input[i])[j..(j + SIZE_XMAS)]
                .iter()
                .collect::<String>()
                .to_lowercase();

            if is_xmas(&word) {
                ans += 1;
            }
        }
    }

    ans
}

fn find_xmas_in_columns(input: &Vec<Vec<char>>) -> i32 {
    let mut ans = 0;

    for i in 0..input[0].len() {
        for j in 0..(input.len() - SIZE_XMAS + 1) {
            let word = vec![
                input[j][i],
                input[j + 1][i],
                input[j + 2][i],
                input[j + 3][i],
            ]
            .iter()
            .collect::<String>()
            .to_lowercase();

            if is_xmas(&word) {
                ans += 1;
            }
        }
    }

    ans
}

fn find_xmas_in_diagonals(input: &Vec<Vec<char>>) -> i32 {
    let mut ans = 0;

    for i in 0..(input.len() - SIZE_XMAS + 1) {
        for j in 0..(input[0].len() - SIZE_XMAS + 1) {
            let word_main = vec![
                input[i][j],
                input[i + 1][j + 1],
                input[i + 2][j + 2],
                input[i + 3][j + 3],
            ]
            .iter()
            .collect::<String>()
            .to_lowercase();

            let word_secondary = vec![
                input[i][input[0].len() - 1 - j],
                input[i + 1][input[0].len() - 2 - j],
                input[i + 2][input[0].len() - 3 - j],
                input[i + 3][input[0].len() - 4 - j],
            ]
            .iter()
            .collect::<String>()
            .to_lowercase();

            if is_xmas(&word_main) {
                ans += 1;
            }

            if is_xmas(&word_secondary) {
                ans += 1;
            }
        }
    }

    ans
}

fn is_xmas(word: &str) -> bool {
    return word == "xmas" || word == "samx";
}

fn find_mas(input: &Vec<Vec<char>>) -> i32 {
    let mut ans = 0;

    for i in 0..(input.len() - SIZE_MAS + 1) {
        for j in 0..(input[0].len() - SIZE_MAS + 1) {
            let word_main = vec![input[i][j], input[i + 1][j + 1], input[i + 2][j + 2]]
                .iter()
                .collect::<String>()
                .to_lowercase();

            let word_secondary = vec![
                input[i][j + SIZE_MAS - 1],
                input[i + 1][j + SIZE_MAS - 2],
                input[i + 2][j],
            ]
            .iter()
            .collect::<String>()
            .to_lowercase();

            if is_mas(&word_main) && is_mas(&word_secondary) {
                ans += 1;
            }
        }
    }

    ans
}

fn is_mas(word: &str) -> bool {
    return word == "mas" || word == "sam";
}
