// use itertools::Itertools;
use std::{collections::{HashMap, HashSet}, env};
use std::time::Instant;
use std::iter::FromIterator;

use aoc2020::util;
use util::read_lines;

fn parse_file(path: &str) -> Vec<i32> {
    read_lines(path)
        .unwrap()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect()
}

fn count_diffs(seq: &[i32]) -> HashMap<i32, i32> {
    let mut out = HashMap::new();
    for i in 1..seq.len() {
        let diff = seq[i - 1] - seq[i];
        *out.entry(diff).or_insert(0) += 1;
    }
    out
}

fn find_seq(jolts: &HashSet<i32>, current: i32, target: i32) -> Option<Vec<i32>> {
    if current == target {
        return Some(vec![]);
    }

    for diff in vec![1, 2, 3] {
        let next = current + diff;
        if next > target {
            continue;
        }
        if jolts.contains(&next) {
            // TODO: use a mask to do this without all the copying
            // Try this as the next item
            let mut next_jolts = jolts.clone();
            next_jolts.remove(&next);
            if let Some(mut seq) = find_seq(&next_jolts, next, target) {
                seq.push(next);
                return Some(seq);
            }
        }
    }

    None
}

fn process_jolts(nums: &[i32]) {
    // XXX what is the .cloned() all about?
    let jolts: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
    let max = nums.iter().max().unwrap();

    let seq = find_seq(&jolts, 0, *max).unwrap();
    let diffs = count_diffs(&seq);
    println!("Sequence: {:?}", seq);
    println!("diffs: {:?}", diffs);
    // TODO: it feels so weird to be taking the address of a constant number.
    //       or do I really really need to read this as "borrow"?
    let a = diffs[&1] + 1;
    let b = diffs[&3] + 1;
    // TODO: ^^^ track down the off-by-one
    println!("answer: {} * {} = {}", a, b, a * b);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    let now = Instant::now();
    let nums = parse_file(&args[1]);
    process_jolts(&nums);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // #[test]
    // fn test_parse_instr() {
    //     assert_eq!(parse_instruction("nop +0"), Op::Nop(0));
    //     assert_eq!(parse_instruction("jmp -4"), Op::Jmp(-4));
    // }
}
