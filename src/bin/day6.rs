use aoc2020::util;
use std::collections::HashMap;
use std::env;

fn count_all_yeses(group: &HashMap<char, u32>, num: u32) -> usize {
    group.iter()
        .filter(|(_k, &v)| v == num)
        .count()
}

fn process_file(path: &str) {
    let mut current_group: HashMap<char, u32> = HashMap::new();
    let mut num_people = 0;
    let mut num_all_yes = 0;

    let lines = util::read_lines(path).unwrap();
    for line_in in lines {
        let line = line_in.unwrap();
        if line == "" {
            num_all_yes += count_all_yeses(&current_group, num_people);
            current_group.clear();
            num_people = 0;
        } else {
            for c in line.chars() {
                *current_group.entry(c).or_insert(0) += 1;
            }
            num_people += 1;
        }
    }
    num_all_yes += count_all_yeses(&current_group, num_people);
    println!("Num all yeses: {}", num_all_yes);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    process_file(&args[1]);
}
