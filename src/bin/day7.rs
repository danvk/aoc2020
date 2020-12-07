#[macro_use]
extern crate lazy_static;

use aoc2020::util;
use std::collections::{HashMap, HashSet};
use std::env;
use regex::Regex;

lazy_static! {
    // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    static ref LINE_RE: Regex = Regex::new(r"([a-z ]+) bags contain (.*)\.$").unwrap();
    // 1 dark olive bag
    // 2 vibrant plum bags
    static ref BAG_RE: Regex = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();
}


fn parse_rules(path: &str) -> HashMap<String, Vec<(String, u32)>> {
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
    bags
}

fn invert_map(input: &HashMap<String, Vec<(String, u32)>>) -> HashMap<String, HashSet<String>> {
    /*
    input
        .iter()
        .flat_map(|(&container, &contents)|
            contents
            .iter()
            .map(|(color, _count)| (color.to_owned(), container.to_owned()))
        )
        .collect()
        */

    let mut out: HashMap<String, HashSet<String>> = HashMap::new();
/*
    for (color, container) in input
    .iter()
    .flat_map(|(&container, &contents)|
        contents
        .into_iter()
        .map(|(color, _count)| (color, String::from(container)))
    ) {
        out.insert(String::from(color), container);
    }
    */
    for (container, contents) in input.into_iter() {
        for (color, _count) in contents.iter() {
            out.entry(String::from(color)).or_insert(HashSet::new()).insert(String::from(container));
        }
    }
    out
}

fn solve_problem(inv_rules: &HashMap<String, HashSet<String>>, start: &str) {
    let mut colors: HashSet<String> = HashSet::new();
    colors.insert(String::from(start));

    loop {
        let mut fringe: HashSet<String> = HashSet::new();
        for color in colors.iter() {
            if inv_rules.contains_key(color) {
                for container in inv_rules[color].iter() {
                    if !colors.contains(&container.to_string()) {
                        fringe.insert(String::from(container));
                    }
                }
            }
        }

        println!("Fringe: {:?}", fringe);
        if fringe.is_empty() {
            break;
        }
        for color in fringe {
            colors.insert(color);
        }
    }

    println!("Containers: {:?}", colors);
    println!("Answer: {}", colors.len() - 1);
}

// not 22

fn process_file(path: &str) {
    let rules = parse_rules(path);
    let inv_rules = invert_map(&rules);
    println!("inverted map: {:?}", inv_rules);
    solve_problem(&inv_rules, "shiny gold");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    process_file(&args[1]);
}
