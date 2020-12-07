#[macro_use]
extern crate lazy_static;

use aoc2020::util;
use std::collections::HashMap;
use std::env;
use regex::Regex;

lazy_static! {
    // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    static ref LINE_RE: Regex = Regex::new(r"([a-z ]+) bags contain (.*)\.$").unwrap();
    // 1 dark olive bag
    // 2 vibrant plum bags
    static ref BAG_RE: Regex = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();
}


fn process_file(path: &str) {
    let mut bags: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    for line_in in util::read_lines(path).unwrap() {
        let line = line_in.unwrap();

        let line_groups = LINE_RE.captures(&line).unwrap();
        let subject = &line_groups[1];
        let contents_str = &line_groups[2];
        if contents_str == "no other bags" {
            bags.insert(String::from(subject), Vec::new());
            continue;
        }

        let bag: Vec<(String, u32)> = contents_str
            .split(", ")
            .map(|part| {
                let cap = BAG_RE.captures(&part).unwrap();
                (String::from(&cap[2]), cap[1].parse::<u32>().unwrap())
            })
            .collect();

        println!("Bag {} contents: {:?}", subject, bag);
        bags.insert(String::from(subject), bag);
    }
    println!("Bags: {:?}", bags);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    process_file(&args[1]);
}
