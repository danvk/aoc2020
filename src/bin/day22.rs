#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use itertools::Itertools;

#[derive(Clone)]
struct GameState {
    p1: Vec<i32>,
    p2: Vec<i32>,
}

impl GameState {
    fn is_done(&self) -> bool {
        self.p1.is_empty() || self.p2.is_empty()
    }

    fn state_str(&self) -> String {
        format!("{};{}",
            self.p1.iter().map(|x| x.to_string()).join(","),
            self.p2.iter().map(|x| x.to_string()).join(","),
        )
    }

    fn print(&self) -> () {
        println!("Player 1: {}\nPlayer 2: {}\n",
            self.p1.iter().map(|x| x.to_string()).join(", "),
            self.p2.iter().map(|x| x.to_string()).join(", "),
        )
    }
}

fn play_one_round(mut state: GameState) -> GameState {
    if state.is_done() {
        return state;
    }

    let c1 = state.p1[0];
    let c2 = state.p2[0];
    if c1 > c2 {
        state.p1.remove(0);
        state.p2.remove(0);
        state.p1.push(c1);
        state.p1.push(c2);
    } else {
        state.p1.remove(0);
        state.p2.remove(0);
        state.p2.push(c2);
        state.p2.push(c1);
    }

    state
}

fn play_game(mut state: GameState) -> Vec<i32> {
    let mut prev_states = HashSet::new();
    while !state.is_done() {
        state.print();
        let state_str = state.state_str();
        if prev_states.contains(&state_str) {
            println!("Same state as before! P1 wins!");
            return state.p1;
        }
        state = play_one_round(state);
        prev_states.insert(state_str);
    }

    if state.p1.is_empty() {
        state.p2
    } else {
        state.p1
    }
}

fn process_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let chunks = contents.split("\n\n").collect::<Vec<_>>();
    assert_eq!(2, chunks.len());

    let p1cards = chunks[0].lines().skip(1).map(|line| line.parse::<i32>().unwrap()).collect_vec();
    let p2cards = chunks[1].lines().skip(1).map(|line| line.parse::<i32>().unwrap()).collect_vec();

    let state = GameState { p1: p1cards, p2: p2cards };

    let winning_hand = play_game(state);
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
