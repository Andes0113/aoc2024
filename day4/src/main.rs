use std::cmp;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() < 2 { "test.txt" } else { &args[1] };

    let input_str: String = fs::read_to_string(file_name).expect("File is read");

    let input: Vec<Vec<char>> = input_str
        .as_str()
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.chars().collect())
        .collect();

    let mut count = 0;
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            count += xmas_count(&input, r, c);
        }
    }
    println!("Answer to part 1: {}", count);

    let mut x_count = 0;
    for r in 1..(input.len() - 1) {
        for c in 1..(input[r].len() - 1) {
            if is_x_mas(&input, r, c) {
                x_count += 1
            }
        }
    }
    println!("Answer to part 2: {}", x_count);
}

fn xmas_count(input: &Vec<Vec<char>>, r_init: usize, c_init: usize) -> i32 {
    let mut count: i32 = 0;
    let xmas = "XMAS";

    let dirs: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    dirs.into_iter().for_each(|(dr, dc)| {
        let mut is_match = true;
        for i in 0..4 {
            let r: i32 = (r_init as i32) + i * dr;
            let c: i32 = (c_init as i32) + i * dc;
            if cmp::min(r, c) < 0 || r >= input.len() as i32 || c >= input[r as usize].len() as i32
            {
                is_match = false;
            }
            if !is_match {
                continue;
            }
            let cur_char = xmas.chars().nth(i as usize).unwrap();
            if input[r as usize][c as usize] != cur_char {
                is_match = false;
            }
        }
        if is_match {
            count += 1;
        }
    });

    count
}

fn is_x_mas(input: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    if input[r][c] != 'A' {
        return false;
    }
    let left_diag = (input[r - 1][c - 1] == 'M' && input[r + 1][c + 1] == 'S')
        || (input[r - 1][c - 1] == 'S' && input[r + 1][c + 1] == 'M');
    let right_diag = (input[r + 1][c - 1] == 'M' && input[r - 1][c + 1] == 'S')
        || (input[r + 1][c - 1] == 'S' && input[r - 1][c + 1] == 'M');

    left_diag && right_diag
}
