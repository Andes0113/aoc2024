use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File is read");

    // println!("With text:\n{contents}");

    let lines = contents.lines();

    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split("   ").collect();
        let str1 = parts[0].to_string();
        let int1 = str1.parse::<i32>().unwrap();

        let str2 = parts[1].to_string();
        let int2 = str2.parse::<i32>().unwrap();
        vec1.push(int1);
        vec2.push(int2);
    }

    vec1.sort();
    vec2.sort();

    let mut diff = 0;

    let mut i = 0;
    while i < vec1.len() {
        diff += (vec1[i] - vec2[i]).abs();
        i += 1;
    }

    println!("Answer to part 1: {}", diff);

    let mut counter: HashMap<i32, i32> = HashMap::new();

    vec2.into_iter().for_each(|x| {
        let new_count = match counter.get(&x) {
            Some(count) => count + 1,
            None => 1
        };
        counter.insert(x, new_count);
    });

    let similarity: i32 = vec1.into_iter().map(|num| {
        num * match counter.get(&num) {
            Some(count) => *count,
            None => 0
        }
    }).sum();

    println!("Answer to part 2: {}", similarity);
}
