use std::env;
use itertools::Itertools;

use aoc2020::util;
use util::read_lines;

fn is_pair_sum(n: u64, nums: &[u64]) -> bool{
    nums.iter().combinations(2).any(|x| x[0] + x[1] == n)
}

fn process_file(path: &str, preamble_len: i32) {
    let nums: Vec<u64> = read_lines(path)
        .unwrap()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();

    let mut i = preamble_len as usize;
    loop {
        i += 1;
        if i >= nums.len() {
            break;
        }

        let n = nums[i];
        if !is_pair_sum(n, &nums[(i as i32 - preamble_len) as usize..i]) {
            println!("Invalid: {}", n);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    let preamble = args[2].parse::<i32>().unwrap();

    process_file(&args[1], preamble);
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
