#[macro_use]
extern crate lazy_static;
use aoc2020::util;
use regex::Regex;
use std::{collections::HashMap, env};
use std::time::Instant;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Op {
    Mask{ones: u64, zeros: u64, xs: Vec<u32>},
    Mem{addr: u64, value: u64},
}

lazy_static! {
    static ref MASK_RE: Regex = Regex::new(r"^mask = ([X01]{36})$").unwrap();
    static ref SET_RE: Regex = Regex::new(r"^mem\[(\d+)] = (\d+)$").unwrap();
}

fn parse_instruction(text: &str) -> Op {
    if let Some(groups) = SET_RE.captures(text) {
        let addr = groups[1].parse::<u64>().unwrap();
        let value = groups[2].parse::<u64>().unwrap();
        return Op::Mem { addr, value }
    }

    if let Some(groups) = MASK_RE.captures(text) {
        let raw = &groups[1];
        let xs: Vec<u32> = raw.chars().enumerate().filter(|&(_i, b)| b == 'X').map(|(i, _b)| (35 - i) as u32).collect();
        let ones = u64::from_str_radix(&raw.chars().map(|b| if b == '1' { '1' } else { '0' }).collect::<String>(), 2).unwrap();
        let zeros = u64::from_str_radix(&raw.chars().map(|b| if b == '0' { '1' } else { '0' }).collect::<String>(), 2).unwrap();

        return Op::Mask { xs, ones, zeros }
    }

    unreachable!("Bad instruction: {}", text);
}

fn read_program(path: &str) -> Vec<Op> {
    util::read_lines(path)
        .unwrap()
        .map(|line| line.unwrap())
        .map(|line| parse_instruction(&line))
        .collect()
}

fn enumerate_xs(xs: &Vec<u32>) -> Vec<u64> {
    let mut out = Vec::new();
    for i in 0..2u32.pow(xs.len() as u32) {
        let mut v = 0u64;
        for k in 0..32 {
            if i & (1 << k) != 0 {
                v += 1u64 << xs[k as usize];
            }
        }
        out.push(v);
    }
    out
}

fn xs_to_mask(xs: &Vec<u32>) -> u64 {
    let mut out = 0u64;
    for x in xs {
        out += 1u64 << x;
    }
    out
}

/// Returns the final accumulator value or None if the program goes into an infinite loop.
fn run_program(ops: &Vec<Op>) -> HashMap<u64, u64> {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    // interesting that you can't make the type a specific variant of the enum
    // let mut mask = Op::Mask { set: 0, mask: 0 };
    let mut cur_ones = 0u64;
    // let mut cur_zeros = 0u64;
    let mut cur_xs: &Vec<u32> = &vec![];

    for op in ops {
        match op {
            Op::Mask { ones, zeros: _, xs } => {
                cur_ones = *ones;
                // cur_zeros = *zeros;
                cur_xs = xs;
            }
            Op::Mem { mut addr, value } => {
                // If the bitmask bit is 0, the corresponding memory address bit is unchanged.
                // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
                // If the bitmask bit is X, the corresponding memory address bit is floating.
                // mem.insert(*addr, (value & cur_xs) | cur_ones);
                // println!("addr: {}", addr);
                addr = (addr & !cur_ones) | cur_ones;
                // println!("addr+ones: {}", addr);
                let mask = xs_to_mask(cur_xs);
                addr = addr & !mask;
                // println!("addr+ones+mask: {}", addr);
                for float in enumerate_xs(cur_xs) {
                    let x = addr | (float & mask);
                    mem.insert(x, *value);
                    // println!("Write @{} value {}", x, *value);
                }
            }
        }
    }

    mem
}

fn process_file(path: &str) {
    let program = read_program(path);
    let now = Instant::now();
    let mem = run_program(&program);
    // println!("Memory: {:?}", mem);
    println!("sum = {}", mem.values().sum::<u64>());
    println!("Time: {}ms", now.elapsed().as_millis());
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

    #[test]
    fn test_enumerate_xs() {
        assert_eq!(enumerate_xs(&vec![0, 1]), vec![0, 1, 2, 3]);
        assert_eq!(enumerate_xs(&vec![0, 2]), vec![0, 1, 4, 5]);
    }
    // #[test]
    // fn test_parse_instr() {
    //     assert_eq!(parse_instruction("mem[7] = 101"), Op::Mem { addr: 7, value: 101 });
    //     assert_eq!(parse_instruction(
    //         "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
    //         //                                   4268421
    //         //                                   631
    //         Op::Mask { xs: 0b111111111111111111111111111110111101, ones: 64, zeros: 2 }
    //     );
    //     // assert_eq!(parse_instruction("jmp -4"), Op::Jmp(-4));
    // }
}
