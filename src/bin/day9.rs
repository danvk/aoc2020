use std::env;
use itertools::Itertools;

use aoc2020::util;
use util::read_lines;

fn is_pair_sum(n: u64, nums: &[u64]) -> bool{
    nums.iter().combinations(2).any(|x| x[0] + x[1] == n)
}

fn parse_file(path: &str) -> Vec<u64> {
    read_lines(path)
    .unwrap()
    .map(|line| line.unwrap().parse::<u64>().unwrap())
    .collect()
}

fn process_file(nums: &[u64], preamble_len: i32) -> Option<u64> {
    let mut i = preamble_len as usize;
    loop {
        i += 1;
        if i >= nums.len() {
            return None;
        }

        let n = nums[i];
        if !is_pair_sum(n, &nums[(i as i32 - preamble_len) as usize..i]) {
            println!("Invalid: {}", n);
            return Some(n);
        }
    }
}

fn find_sequence(nums: &[u64], n: u64) {
    let m = (0..nums.len()).combinations(2).find(|x| {
        nums[x[0]..=x[1]].iter().sum::<u64>() == n
    });
    let indices = m.unwrap();
    let a = indices[0];
    let b = indices[1];
    let s = &nums[a..=b];
    let lo = s.iter().min().unwrap();
    let hi = s.iter().max().unwrap();
    println!("a: {}, b: {}, min: {}, max: {}, sum: {}", a, b, lo, hi, lo + hi);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    let preamble = args[2].parse::<i32>().unwrap();
    let nums = parse_file(&args[1]);

    let invalid = process_file(&nums, preamble).unwrap();
    find_sequence(&nums, invalid);
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