use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() < 2 { "test.txt" } else { &args[1] };

    let input: String = fs::read_to_string(file_name).expect("File is read");

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut res = 0;

    mul_regex.captures_iter(&input).for_each(|caps| {
        let num1 = caps
            .get(1)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let num2 = caps
            .get(2)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();

        res += num1 * num2;
    });

    println!("Answer to part 1: {}", res);
}
