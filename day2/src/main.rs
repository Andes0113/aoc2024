use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() < 2 { "test.txt" } else { &args[1] };

    let input = fs::read_to_string(file_name).expect("Error reading file");

    let lines = input.lines();

    let num_safe = lines.map(|line| {
        line.split(" ").map(|c| c.to_string().parse::<i32>().unwrap()).collect()
    }).filter(|reports: &Vec<i32>| {
        if reports.len() <= 1 {
            return true
        }

        let mut i = 1;
        let mut increasing = false;

        while i < reports.len() {
            let diff = reports[i] - reports[i - 1];
            if diff == 0 || diff.abs() > 3 {
                return false
            }
            let moment_is_increase = diff > 0;
            if i == 1 {
                increasing = moment_is_increase
            } else if moment_is_increase != increasing {
                return false
            }
            i += 1;
        }
        
        true
    }).count();



    println!("(Part 1) Safe Count: {}", num_safe);

}

