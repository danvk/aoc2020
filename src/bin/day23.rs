use std::{fmt, time::Instant};
use std::env;
use itertools::Itertools;

struct Cups {
    // 0 is empty
    nexts: Vec<usize>,
    current: usize,
}

impl Cups {
    fn remove_three_after(&mut self, c0: usize) -> (usize, usize, usize) {
        let c1 = self.nexts[c0];
        let c2 = self.nexts[c1];
        let c3 = self.nexts[c2];
        let c4 = self.nexts[c3];
        self.nexts[c0] = c4;

        (c1, c2, c3)
    }

    fn insert_after(&mut self, c0: usize, val: usize) -> () {
        let old_next = self.nexts[c0];
        self.nexts[c0] = val;
        self.nexts[val] = old_next;
    }

    fn find_dest(&self, cur: usize, (c1, c2, c3): (usize, usize, usize)) -> usize {
        // println!("find_dest({}, {:?})", cur, (c1, c2, c3));
        let mut dest = cur - 1;
        if dest <= 0 {
            dest += self.nexts.len() - 1;
        }

        if dest != c1 && dest != c2 && dest != c3 {
            return dest;
        }

        self.find_dest(dest, (c1, c2, c3))
    }

    fn play_one_round(&mut self) -> () {
        // println!("cups: {}", self);
        let (c1, c2, c3) = self.remove_three_after(self.current);
        // println!("pick up: {}, {}, {}", c1, c2, c3);
        let dest = self.find_dest(self.current, (c1, c2, c3));
        // println!("destination: {}", dest);

        self.insert_after(dest, c3);
        self.insert_after(dest, c2);
        self.insert_after(dest, c1);

        self.current = self.nexts[self.current];
    }
}


impl fmt::Display for Cups {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::with_capacity(self.nexts.len() * 2 + 2);
        s.push_str(&format!("({})", self.current));
        let mut next = self.nexts[self.current];
        let mut x = 0;
        while next != self.current && x < 25 {
            s.push_str(" ");
            s.push_str(&format!("{}", next));
            next = self.nexts[next];
            x += 1;
        }
        write!(f, "{}", s)
    }
}

fn play_game(nums: Vec<usize>, num_rounds: usize) {
    let mut nexts = vec![0; 1_000_001];
    for (&v0, &v1) in nums.iter().zip(nums.iter().skip(1)) {
        nexts[v0] = v1;
    }
    nexts[*nums.last().unwrap()] = nums.len() + 1;

    for n in nums.len() + 1..=1_000_000 {
        nexts[n] = n + 1;
    }
    nexts[1_000_000] = nums[0];

    let mut cups = Cups { current: nums[0] as usize, nexts };
    println!("Cups: {}", cups);

    let now = Instant::now();
    for _i in 1..=num_rounds {
        // println!("-- move {}--", i);
        cups.play_one_round();
    }
    let elapsed_ms = now.elapsed().as_millis();
    println!("{} rounds in {}ms = {}ms/round", num_rounds, elapsed_ms, elapsed_ms as f64 / num_rounds as f64);

    println!("cups: {}", cups);
    let c1 = cups.nexts[1];
    let c2 = cups.nexts[c1];
    println!("two right of 1: {} * {} = {}",
        c1, c2, c1 * c2,
    );

    // println!("-- final --");
    // println!("cups: {}", cups);

    // println!("answer: {}", cups.answer());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Expected two argument, got {}: {:?}", args.len(), args);
    }

    let cups = args[1].split("").filter(|&s| s != "").map(|c| c.parse::<usize>().unwrap()).collect_vec();
    let num_rounds = args[2].parse::<usize>().unwrap();

    play_game(cups, num_rounds);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

}

// ideas:
// - use a better structure (vecdeque? linked list?)
// - figure out a math-y way to do it
