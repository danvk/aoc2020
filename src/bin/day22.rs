#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use itertools::Itertools;

fn play_game(p1cards: &Vec<i32>, p2cards: &Vec<i32>) -> Vec<i32> {
    let mut p1cards = p1cards.clone();
    let mut p2cards = p2cards.clone();

    while !p1cards.is_empty() && !p2cards.is_empty() {
        let c1 = p1cards[0];
        let c2 = p2cards[0];
        if c1 > c2 {
            p1cards.remove(0);
            p2cards.remove(0);
            p1cards.push(c1);
            p1cards.push(c2);
        } else {
            p1cards.remove(0);
            p2cards.remove(0);
            p2cards.push(c2);
            p2cards.push(c1);
        }
        // println!("p1: {:?}", p1cards);
        // println!("p2: {:?}", p2cards);
    }

    if p1cards.is_empty() {
        p2cards
    } else {
        p1cards
    }
}

fn process_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let chunks = contents.split("\n\n").collect::<Vec<_>>();
    assert_eq!(2, chunks.len());

    let p1cards = chunks[0].lines().skip(1).map(|line| line.parse::<i32>().unwrap()).collect_vec();
    let p2cards = chunks[1].lines().skip(1).map(|line| line.parse::<i32>().unwrap()).collect_vec();

    let winning_hand = play_game(&p1cards, &p2cards);
    println!("Winning hand: {:?}", winning_hand);
    let n = winning_hand.len() as i32;
    println!("answer: {}", winning_hand.iter().enumerate().map(|(i, card)| (n - i as i32) * card).sum::<i32>());
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
