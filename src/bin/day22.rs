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

    let p1 = &mut state.p1;
    let p2 = &mut state.p2;

    let c1 = p1[0];
    let c2 = p2[0];

    // If both players have at least as many cards remaining in their deck as the value
    // of the card they just drew, the winner of the round is determined by playing a
    // new game of Recursive Combat (see below).
    let round_winner;
    if p1.len() as i32 >= c1 + 1 && p2.len() as i32 >= c2 + 1 {
        // recursive combat
        let recur_state = GameState {
            p1: p1[1usize..=c1 as usize].iter().map(|c| *c).collect_vec(),
            p2: p2[1usize..=c2 as usize].iter().map(|c| *c).collect_vec(),
        };
        // println!("recursing");
        // recur_state.print();
        let (who, _cards) = play_game(recur_state);
        round_winner = who;
    } else if c1 > c2 {
        round_winner = 1;
    } else {
        round_winner = 2;
    }

    if round_winner == 1 {
        // println!("Player 1 wins this round");
        p1.remove(0);
        p2.remove(0);
        p1.push(c1);
        p1.push(c2);
    } else {
        // println!("Player 2 wins this round");
        p1.remove(0);
        p2.remove(0);
        p2.push(c2);
        p2.push(c1);
    }

    state
}

fn play_game(mut state: GameState) -> (i32, Vec<i32>) {
    let mut prev_states = HashSet::new();
    while !state.is_done() {
        // state.print();
        let state_str = state.state_str();
        if prev_states.contains(&state_str) {
            // println!("Same state as before! P1 wins!");
            return (1, state.p1);
        }
        state = play_one_round(state);
        prev_states.insert(state_str);
    }

    if state.p1.is_empty() {
        (2, state.p2)
    } else {
        (1, state.p1)
    }
}

fn process_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let chunks = contents.split("\n\n").collect::<Vec<_>>();
    assert_eq!(2, chunks.len());

    let p1cards = chunks[0].lines().skip(1).map(|line| line.parse::<i32>().unwrap()).collect_vec();
    let p2cards = chunks[1].lines().skip(1).map(|line| line.parse::<i32>().unwrap()).collect_vec();

    let state = GameState { p1: p1cards, p2: p2cards };

    let (who, winning_hand) = play_game(state);
    println!("Winner: player {}", who);
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
