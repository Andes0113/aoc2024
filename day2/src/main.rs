use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() < 2 { "test.txt" } else { &args[1] };

    let input = fs::read_to_string(file_name).expect("File is read");

    let lines = input.lines();

    let reports: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split(" ")
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let num_safe = reports.clone().into_iter().filter(is_safe).count();

    // Scuffed solution, enumerating possibilities
    let num_safe_dampened = reports
        .into_iter()
        .map(|levels: Vec<i32>| {
            for i in 0..levels.len() {
                let mut level_subarr = levels.clone();
                level_subarr.remove(i);
                if is_safe(&level_subarr) {
                    return true;
                }
            }
            false
        })
        .filter(|cond| *cond)
        .count();

    println!("(Part 1) Safe Count: {}", num_safe);
    println!("(Part 2) Safe Count: {}", num_safe_dampened);
}

fn is_safe(levels: &Vec<i32>) -> bool {
    {
        let mut increasing = false;

        for i in 1..levels.len() {
            let diff = levels[i] - levels[i - 1];
            if diff == 0 || diff.abs() > 3 {
                return false;
            }
            let moment_is_increase = diff > 0;
            if i == 1 {
                increasing = moment_is_increase
            } else if moment_is_increase != increasing {
                return false;
            }
        }

        true
    }
}
