#[macro_use]
extern crate lazy_static;
use aoc2020::util;
use regex::Regex;
use std::{collections::HashSet, env};


#[derive(PartialEq, Eq, Clone, Debug)]
enum Cell {
    Empty,
    Occupied,
    Floor,
}

fn parse_char(c: char) -> Cell {
    match c {
        '.' => Cell::Floor,
        '#' => Cell::Occupied,
        'L' => Cell::Empty,
        _ => panic!("Invalid cell: {}", c)
    }
}

fn parse_ferry(path: &str) -> Vec<Vec<Cell>> {
    util::read_lines(path)
    .unwrap()
    .map(|line| line.unwrap())
    .map(|line| line.chars().map(parse_char).collect())
    .collect()
}

fn process_file(path: &str) {
    let ferry = parse_ferry(path);
    println!("Ferry: {:?}", ferry);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    process_file(&args[1]);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

}
