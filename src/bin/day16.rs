#[macro_use]
extern crate lazy_static;
use aoc2020::util;
use regex::Regex;
use std::collections::HashMap;
use std::env;

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
}

fn parse_rules(rules: &str) -> Vec<bool> {
    let mut ok = vec![false; 1000];
    for line in rules.split('\n') {
        let caps = RULE_RE.captures(line).unwrap();
        let low0 = caps[2].parse::<i32>().unwrap();
        let hi0 = caps[3].parse::<i32>().unwrap();
        let low1 = caps[4].parse::<i32>().unwrap();
        let hi1 = caps[5].parse::<i32>().unwrap();
        for i in low0..=hi0 {
            ok[i as usize] = true;
        }
        for i in low1..=hi1 {
            ok[i as usize] = true;
        }
    }
    ok
}

fn process_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let chunks = contents.split("\n\n").collect::<Vec<_>>();

    let ok_nums = parse_rules(&chunks[0]);
    println!("num set: {}", ok_nums.iter().filter(|&&x| x).count());

    let mut sum_invalid = 0;
    for line in chunks[2].split('\n').into_iter().skip(1) {
        if line.is_empty() {
            continue;
        }
        sum_invalid += line.split(',').map(|x| x.parse::<i32>().unwrap())
        .filter(|&x| !ok_nums[x as usize]).sum::<i32>();
    }
    println!("Sum invalid: {}", sum_invalid);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    process_file(&args[1]);
}
