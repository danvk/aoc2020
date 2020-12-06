use aoc2020::util;
use std::collections::HashMap;
use std::env;

fn count_all_yeses(group: &HashMap<char, u32>, num: u32) -> usize {
    group.iter()
        .filter(|(_k, &v)| v == num)
        .count()
}

// TODO: factor out a read_chunks function
// TODO: make the `for c in line.chars()` bit add two HashMaps together

fn process_file(path: &str) {
    let mut num_all_yes = 0;

    // for chunk in util::read_chunks(path).unwrap() {
    for chunk in std::fs::read_to_string(path).unwrap().split("\n\n") {
        let mut current_group: HashMap<char, u32> = HashMap::new();
        let mut num_people = 0;

        for line in chunk.lines() {
            for c in line.chars() {
                *current_group.entry(c).or_insert(0) += 1;
            }
            num_people += 1;
        }
        num_all_yes += count_all_yeses(&current_group, num_people);
    }
    println!("Num all yeses: {}", num_all_yes);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    process_file(&args[1]);
}
