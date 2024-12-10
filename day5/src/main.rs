use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut predecessors : HashMap<i32, HashSet<i32>> = Default::default();
    let mut descendants : HashMap<i32, HashSet<i32>> = Default::default();

    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() < 2 { "test.txt" } else { &args[1] };

    let input_str: String = fs::read_to_string(file_name).expect("File is read");

    // Get all lines from input
    let lines : Vec<&str> = input_str.split("\n").collect();

    let mut i = 0;

    // Get list of rules
    while lines[i].len() > 0 {
        let pair : Vec<&str> = lines[i].split("|").collect();
        let predec: i32 = pair[0].to_string().parse::<i32>().unwrap();
        let page: i32 = pair[1].to_string().parse::<i32>().unwrap();

        if !predecessors.contains_key(&page) {
            predecessors.insert(page, HashSet::new());
        }
        if !descendants.contains_key(&predec) {
            descendants.insert(predec, HashSet::new());
        }
        predecessors.get_mut(&page).unwrap().insert(predec);
        descendants.get_mut(&predec).unwrap().insert(page);

        i += 1
    }
    i += 1;

    let mut part1: f64 = 0.;
    let mut part2: f64 = 0.;

    // Iterate through update lists
    while i < lines.len() && lines[i].len() > 0 {
        let pages: Vec<i32> = lines[i].split(",").map(|num| num.parse::<i32>().unwrap()).collect();
        if in_order(&pages, &predecessors) {
            part1 += median(&pages);
        } else {
            part2 += median(&ordered(&pages, &predecessors, &descendants));
        }
        i += 1
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn in_order(pages: &Vec<i32>, predecessors: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut prev : HashSet::<i32> = Default::default();
    // Banned: predecessors for pages we've already seen
    let mut banned : HashSet::<i32> = Default::default();
    for page in pages {
        if banned.contains(&page) {
            return false;
        }

        if predecessors.contains_key(&page) {
            predecessors[&page].iter().for_each(|pred| {
                if !prev.contains(pred) {
                    banned.insert(*pred);
                }
            });
        }
        prev.insert(*page);
    }
    true
}

fn median(pages: &Vec<i32>) -> f64 {
    if pages.len() % 2 == 1 {
        pages[pages.len() / 2] as f64
    } else {
        let left = pages[pages.len() / 2 - 1];
        let right = pages[pages.len() / 2];
        println!("med: {} {}", left, right);
        f64::from(left + right) / 2.0
    }
}

// Topological sort
fn ordered(pages: &Vec<i32>, predecessors: &HashMap<i32, HashSet<i32>>, descendants: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut rearranged : Vec<i32> = vec![];
    let mut unused : HashSet<i32> = HashSet::from_iter(pages.clone());

    let mut available : Vec<i32> = vec![];

    // Get all pages without dependencies
    for page in pages {
        if !predecessors.contains_key(&page) || predecessors[&page].intersection(&unused).count() == 0 {
            available.push(*page);
        }
    }

    // Continuously get pages that have no unused predecessors and add them to rearranged
    while !available.is_empty() {
        let curr = available.pop().unwrap();
        rearranged.push(curr);
        unused.remove(&curr);
        if !descendants.contains_key(&curr) {
            continue;
        }
        descendants[&curr].iter().for_each(|desc| {
            // If a predecessor hasn't been used and has no predecessors, add it to the list
            if unused.contains(desc) && (!predecessors.contains_key(&desc) || predecessors[&desc].intersection(&unused).count() == 0) {
                available.push(*desc);
            }
        });
    }

    rearranged
}