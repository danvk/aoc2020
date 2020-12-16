#[macro_use]
extern crate lazy_static;
use aoc2020::util;
use regex::Regex;
use std::{collections::{HashMap, HashSet}, ops::RangeInclusive};
use std::env;

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
}

#[derive(Debug)]
struct Rule {
    name: String,
    r1: RangeInclusive<i32>,
    r2: RangeInclusive<i32>,
}

impl Rule {
    fn contains(&self, num: i32) -> bool {
        return self.r1.contains(&num) || self.r2.contains(&num);
    }
}

fn parse_rule(rule: &str) -> Rule {
    let caps = RULE_RE.captures(rule).unwrap();
    let name = String::from(&caps[1]);
    let low0 = caps[2].parse::<i32>().unwrap();
    let hi0 = caps[3].parse::<i32>().unwrap();
    let low1 = caps[4].parse::<i32>().unwrap();
    let hi1 = caps[5].parse::<i32>().unwrap();

    Rule {
        name,
        r1: low0..=hi0,
        r2: low1..=hi1,
    }
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    rules.split('\n').map(|rule| parse_rule(rule)).collect()
}

fn parse_ticket(ticket: &str) -> Vec<i32> {
    ticket.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

fn determine_rules(rules: &Vec<Rule>, tickets: &Vec<Vec<i32>>) -> HashMap<String, usize> {
    let mut possible_rules: Vec<(&str, HashSet<usize>)> = vec![];

    for rule in rules.iter() {
        let poss = (0..rules.len()).filter(|&i| tickets.iter().all(|ticket| rule.contains(ticket[i]))).collect::<HashSet<_>>();
        possible_rules.push((&rule.name, poss));
    }

    // XXX why do you have to unwrap partial_cmp()?
    possible_rules.sort_by(|a, b| b.1.len().partial_cmp(&a.1.len()).unwrap());

    // println!("possible rules: {:?}", possible_rules);

    let mut result: HashMap<String, usize> = HashMap::new();
    while !possible_rules.is_empty() {
        let (name, poss) = possible_rules.pop().unwrap();
        if poss.len() != 1 {
            panic!("Too many possibilities: {} {:?}", name, poss);
        }
        let i = poss.iter().next().unwrap();
        result.insert(String::from(name), *i);

        for (_, p) in possible_rules.iter_mut() {
            p.remove(i);
        }
    }

    result
}

fn process_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let chunks = contents.split("\n\n").collect::<Vec<_>>();

    let rules = parse_rules(&chunks[0]);
    println!("Rules: {:?}", rules);

    let mut ok_tickets: Vec<Vec<i32>> = vec![];

    let my_ticket = parse_ticket(chunks[1].split('\n').skip(1).next().unwrap());
    ok_tickets.push(my_ticket.clone());

    for line in chunks[2].split('\n').into_iter().skip(1) {
        if line.is_empty() {
            continue;
        }
        let ticket = parse_ticket(line);
        let is_ok = ticket.iter().all(|&num|
            rules.iter().any(|rule| rule.contains(num)));
        if is_ok {
            ok_tickets.push(ticket);
        }
    }

    // println!("OK tickets: {:?}", ok_tickets);

    let rule_indices = determine_rules(&rules, &ok_tickets);
    println!("Rule indices: {:?}", rule_indices);

    let mut result = 1u64;
    for (n, &i) in rule_indices.iter() {
        if n.starts_with("departure") {
            println!("{}: index {} value {}", n, i, my_ticket[i]);
            result *= my_ticket[i] as u64;
        }
    }
    println!("Result: {}", result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    process_file(&args[1]);
}
