use std::{collections::{HashMap, HashSet}, fmt};
use std::env;
use itertools::Itertools;

#[derive(Clone)]
struct Cups {
    cups: Vec<i32>,
    current: usize,
}

impl Cups {
    fn idx(&self, i: i32) -> usize {
        // TODO: there's a euclid_rem method
        let len = self.cups.len() as i32;
        (((i % len) + len) % len) as usize
    }

    fn at(&self, i: i32) -> i32 {
        self.cups[self.idx(i)]
    }

    fn max_index(&self) -> usize {
        self.cups.iter().position_max().unwrap()
    }

    fn index_of(&self, i: i32) -> Option<usize> {
        self.cups.iter().position(|&n| n == i)
    }

    fn remove_three_at(&mut self, i: usize) -> (i32, i32, i32) {
        let mut p = self.idx(i as i32);
        let c1 = self.cups.remove(p);
        p = self.idx(p as i32);
        let c2 = self.cups.remove(p);
        p = self.idx(p as i32);
        let c3 = self.cups.remove(p);
        (c1, c2, c3)
    }

    fn play_one_round(&mut self) -> () {
        println!("cups: {}", self);
        let cur_val = self.cups[self.current as usize];
        let (c1, c2, c3) = self.remove_three_at(self.current + 1);
        println!("pick up: {}, {}, {}", c1, c2, c3);

        let mut d_opt = None;
        let min = *self.cups.iter().min().unwrap();
        for i in 1..self.cups.len() as i32 {
            let v = cur_val - i;
            if v < min {
                break;
            }
            if let Some(dest) = self.index_of(v) {
                d_opt = Some(dest);
                break;
            }
        }
        if d_opt.is_none() {
            d_opt = Some(self.max_index() as usize);
        }
        let d = d_opt.unwrap();
        println!("destination: {}", self.cups[d]);

        let nd = self.idx(d as i32 + 1);
        self.cups.insert(nd, c3);
        self.cups.insert(nd, c2);
        self.cups.insert(nd, c1);

        self.current = self.idx(self.index_of(cur_val).unwrap() as i32 + 1);
    }

    fn answer(&self) -> String {
        let i = self.index_of(1).unwrap();
        (1..self.cups.len()).map(|d| self.at((i + d) as i32).to_string()).collect::<String>()
    }
}


impl fmt::Display for Cups {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: rewrite w/ join / collect::<String>
        let mut s = String::with_capacity(self.cups.len() * 2 + 2);
        for (i, cup) in self.cups.iter().enumerate() {
            if i > 0 {
                s.push_str(" ");
            }
            if i == self.current {
                s.push_str(&format!("({})", cup));
            } else {
                s.push_str(&format!("{}", cup));
            }
        }
        write!(f, "{}", s)
    }
}

fn play_game(nums: Vec<i32>, num_rounds: usize) {
    let mut cups = Cups { current: 0, cups: nums };

    for i in 1..=num_rounds {
        println!("-- move {}--", i);
        cups.play_one_round();
    }

    println!("-- final --");
    println!("cups: {}", cups);

    println!("answer: {}", cups.answer());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Expected two argument, got {}: {:?}", args.len(), args);
    }

    let cups = args[1].split("").filter(|&s| s != "").map(|c| c.parse::<i32>().unwrap()).collect_vec();
    let num_rounds = args[2].parse::<usize>().unwrap();

    play_game(cups, num_rounds);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

}
