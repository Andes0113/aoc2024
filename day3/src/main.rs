use regex::Captures;
use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() < 2 { "test.txt" } else { &args[1] };

    let input: String = fs::read_to_string(file_name).expect("File is read");

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut res = 0;

    // Use regex to iteratively process each mul command
    mul_regex
        .captures_iter(&input)
        .map(get_capture_product)
        .for_each(|prod| res += prod);

    println!("Answer to part 1: {}", res);

    let mut part2_res: i32 = 0;

    let mut input_mut = input;
    let mut enabled = true;

    while input_mut.len() > 0 {
        if !enabled {
            // If not enabled, find next instance of do() and begin parsing rest as enabled
            let substr_idx_opt: Option<usize> = input_mut.find("do()");
            if substr_idx_opt.is_none() {
                break;
            };
            let substr_idx = substr_idx_opt.unwrap();
            input_mut = input_mut[substr_idx + "do()".len()..].to_string();
            enabled = true;
        } else {
            // Find next instance of mul command
            let borrowed_input = input_mut.clone(); // Clone to navigate around borrow checker
            let mul_opt: Option<regex::Match<'_>> = mul_regex.find(&borrowed_input);
            let dont_opt = input_mut.find("don't()");
            if mul_opt.is_none() {
                // If no mul commands are found in the rest of the string, we're done
                break;
            }

            let mul = mul_opt.unwrap();

            if dont_opt.is_some() {
                let dont = dont_opt.unwrap();

                // If we find dont, check if it occurs before our next mul
                if dont < mul.start() {
                    // If it does, disable and skip processing mul
                    enabled = false;
                    input_mut = input_mut[dont + "don't".len()..].to_string();
                    continue;
                }
            }

            // Parse this mul and add its product to our result
            part2_res += get_capture_product(mul_regex.captures(mul.as_str()).unwrap());

            input_mut = input_mut[mul.end()..].to_string();
        }
    }

    println!("Answer to part 2: {}", part2_res);
}

fn get_capture_product(caps: Captures<'_>) -> i32 {
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

    num1 * num2
}
