#[macro_use] extern crate lazy_static;
use std::collections::HashMap;

use regex::Regex;
use aoc2020::util;

struct Password {
  password: String,
  policy_char: char,
  min: u32,
  max: u32
}

fn read_password(path: &str) -> Password {
  lazy_static! {
    static ref RE: Regex = Regex::new("^(\\d+)-(\\d+) ([a-z]): ([a-z]+)$").unwrap();
  }
  // vscode isn't able to follow along with the types for RE
  // let re: Regex = Regex::new("^(\\d+)-(\\d+): ([a-z]+)$").unwrap();

  let groups = RE.captures(path).unwrap();
  let min = groups[1].parse::<u32>().unwrap();
  let max = groups[2].parse::<u32>().unwrap();
  let policy_char = groups[3].parse::<char>().unwrap();
  let password = String::from(&groups[4]);

  Password{password, policy_char, min, max}
}

fn count_letters(txt: &str) -> HashMap<char, u32> {
  let mut counts = HashMap::new();
  for c in txt.chars() {
    *counts.entry(c).or_insert(0) += 1;
  }

  counts
}

fn is_valid_password(pass: &Password) -> bool {
  let Password {min, max, policy_char, password} = pass;

  let c = *policy_char;
  // println!("{}, {}: {}", min, max, password);
  let c1 = password.chars().nth((*min - 1) as usize).unwrap();
  let c2 = password.chars().nth((*max - 1) as usize).unwrap();

  (if c1 == c { 1 } else { 0 }) + (if c2 == c { 1 } else { 0 }) == 1

  // let counts = count_letters(&password);

  // let count = *counts.get(policy_char).unwrap_or(&0);

  // for count in counts.values() {
  //   if count < min || count > max {
  //     return false;
  //   }
  // }
}

fn process_file(path: &str) {
  let lines = util::read_lines(path).unwrap();

  let mut num_valid = 0;
  let mut num_invalid = 0;

  for line in lines {
    let txt = line.unwrap();
    let pass = read_password(&txt);
    if is_valid_password(&pass) {
      num_valid += 1;
    } else {
      num_invalid += 1;
    }
  }

  println!("{} valid, {} invalid passwords", num_valid, num_invalid);
}

fn main() {
  process_file("./inputs/day2.txt");
}
