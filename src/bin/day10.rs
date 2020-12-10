// use itertools::Itertools;
use std::{collections::HashMap, env};
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

fn find_seq(jolts: &[i32], current: i32, target: i32) -> Option<Vec<i32>> {
    if current == target {
        return Some(vec![]);
    }

    for i in 0..jolts.len().min(3) {
        let next = jolts[i];
        if next > target || next - current > 3 {
            continue;
        }
        if let Some(mut seq) = find_seq(&jolts[(1+i)..], next, target) {
            seq.push(next);
            return Some(seq);
        }
    }

    None
}

fn count_distinct(jolts: &[i32]) -> i64 {
    if jolts.len() == 0 {
        return 0;
    }
    if jolts.len() == 1 {
        return 1;
    }

    let current = *jolts.first().unwrap();
    let target = *jolts.last().unwrap();
    let mut num_distinct = 0;
    for i in 1..jolts.len().min(4) {
        let next = jolts[i];
        if next > target || next - current > 3 {
            continue;
        }
        if next == target {
            num_distinct += 1;
        } else {
            num_distinct += count_distinct(&jolts[i..]);
        }
    }

    num_distinct
}

fn count_distinct_fancy(jolts: &[i32]) -> i64 {
    let mut mandatory: Vec<usize> = (1..jolts.len()).filter(|&i| jolts[i] - jolts[i - 1] == 3).collect();
    if *mandatory.last().unwrap() != jolts.len() - 1 {
        mandatory.push(jolts.len() - 1);
    }
    if mandatory[0] != 0 {
        mandatory.insert(0, 0);
    }

    println!("jolts: {:?}", jolts);
    println!("mandatory: {:?}", mandatory);

    let ways = mandatory.windows(2).map(|x| {
        let i = x[0];
        let j = x[1];
        let n = count_distinct(&jolts[i..=j]);
        println!("({}, {}) count_distinct: {:?}, cur={}, target={} --> {}",
        i, j,
        &jolts[i..=j], jolts[i], jolts[j], n);
        n
    });

    ways.product()
}

fn process_jolts(nums: &[i32]) {
    // TODO: why do I need the .map() here?
    let mut jolts: Vec<i32> = Vec::from_iter(nums.iter().map(|x| *x));
    jolts.insert(0, 0);
    jolts.sort();
    let max = jolts.iter().max().unwrap();
    println!("Jolts: {:?}", jolts);

    let seq = find_seq(&jolts, 0, *max).unwrap();
    let diffs = count_diffs(&seq);
    println!("Sequence: {:?}", seq);
    println!("len = {}", seq.len());
    println!("max = {}", max);
    println!("diffs: {:?}", diffs);
    // TODO: it feels so weird to be taking the address of a constant number.
    //       or do I really really need to read this as "borrow"?
    let a = diffs[&1] + 1;
    let b = diffs[&3] + 1;
    // TODO: ^^^ track down the off-by-one
    println!("answer: {} * {} = {}", a, b, a * b);

    let distinct = count_distinct_fancy(&jolts);
    println!("distinct ways: {}", distinct);
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

    #[test]
    fn test_count_distinct() {
        assert_eq!(count_distinct(&vec![14, 14]), 1);
        assert_eq!(count_distinct(&vec![14, 17]), 1);
        assert_eq!(count_distinct(&vec![14, 17, 20]), 1);
        assert_eq!(count_distinct(&vec![14, 15, 17, 20]), 2);
    }

    #[test]
    fn test_count_distinct4() {
        assert_eq!(count_distinct(&vec![0, 1, 2, 3]), 4);
    }
}
