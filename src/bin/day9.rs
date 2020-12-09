use itertools::Itertools;
use std::env;
use std::time::Instant;

use aoc2020::util;
use util::read_lines;

fn is_pair_sum(n: u64, nums: &[u64]) -> bool {
    nums.iter().tuple_combinations().any(|(a, b)| a + b == n)
}

fn parse_file(path: &str) -> Vec<u64> {
    read_lines(path)
        .unwrap()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect()
}

fn process_file(nums: &[u64], preamble_len: usize) -> Option<u64> {
    let res = nums
        .iter()
        .enumerate()
        .dropping(preamble_len)
        .find(|(i, &n)| !is_pair_sum(n, &nums[(*i as i32 - preamble_len as i32) as usize..*i]))
        .map(|x| *x.1);
    res

    // for i in (preamble_len as usize)..nums.len() {
    //     let n = nums[i];
    //     if !is_pair_sum(n, &nums[(i as i32 - preamble_len) as usize..i]) {
    //         println!("Invalid: {}", n);
    //         return Some(n);
    //     }
    // }
    // None
}

fn find_sequence(nums: &[u64], target: u64) {
    let n = nums.len() as i32;
    let (a, b) = (2..n)
        .find_map(|d| {
            (0..n - d)
                .find(|&a| nums[a as usize..(a + d) as usize].iter().sum::<u64>() == target)
                .map(|a| (a, a + d))
        })
        .unwrap();

    let s = &nums[a as usize..=b as usize];
    let lo = s.iter().min().unwrap();
    let hi = s.iter().max().unwrap();
    println!(
        "a: {}, b: {}, min: {}, max: {}, sum: {}",
        a,
        b,
        lo,
        hi,
        lo + hi
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    let now = Instant::now();
    let preamble = args[2].parse::<usize>().unwrap();
    let nums = parse_file(&args[1]);

    let invalid = process_file(&nums, preamble).unwrap();
    println!("Invalid number: {}", invalid);
    find_sequence(&nums, invalid);
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
