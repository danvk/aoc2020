#[macro_use]
extern crate lazy_static;
use aoc2020::util;
use regex::Regex;
use std::{collections::{HashMap, HashSet}, env};

// TODO: x Change Instruction.op to be more like a tagged union
// TODO: x Use match / case on op.op
// TODO: x Use if let in process_file

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Op {
    Mask{ones: u64, zeros: u64, xs: u64},
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
        let xs = u64::from_str_radix(&raw.chars().map(|b| if b == 'X' { '1' } else { '0' }).collect::<String>(), 2).unwrap();
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

/// Returns the final accumulator value or None if the program goes into an infinite loop.
fn run_program(ops: &Vec<Op>) -> HashMap<u64, u64> {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    // interesting that you can't make the type a specific variant of the enum
    // let mut mask = Op::Mask { set: 0, mask: 0 };
    let mut cur_ones = 0u64;
    let mut _cur_zeros = 0u64;
    let mut cur_xs = 0u64;

    for op in ops {
        match op {
            Op::Mask { ones, zeros, xs } => {
                cur_ones = *ones;
                _cur_zeros = *zeros;
                cur_xs = *xs;
            }
            Op::Mem { addr, value } => {
                mem.insert(*addr, (value & cur_xs) | cur_ones);
            }
        }
    }

    mem
}

fn process_file(path: &str) {
    let program = read_program(path);
    let mem = run_program(&program);
    println!("Memory: {:?}", mem);
    println!("sum = {}", mem.values().sum::<u64>());
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
    fn test_parse_instr() {
        assert_eq!(parse_instruction("mem[7] = 101"), Op::Mem { addr: 7, value: 101 });
        assert_eq!(parse_instruction(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            //                                   4268421
            //                                   631
            Op::Mask { xs: 0b111111111111111111111111111110111101, ones: 64, zeros: 2 }
        );
        // assert_eq!(parse_instruction("jmp -4"), Op::Jmp(-4));
    }
}
