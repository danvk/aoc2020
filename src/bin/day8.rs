#[macro_use]
extern crate lazy_static;
use aoc2020::util;
use regex::Regex;
use std::{collections::HashSet, env};

// TODO: x Change Instruction.op to be more like a tagged union
// TODO: x Use match / case on op.op
// TODO: x Use if let in process_file

#[derive(PartialEq, Eq, Clone, Debug)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

lazy_static! {
    static ref INSTR_RE: Regex = Regex::new(r"^([a-z]+) ([-+])([0-9]+)$").unwrap();
}

fn parse_instruction(text: &str) -> Op {
    let groups = INSTR_RE.captures(text).unwrap();
    let op = &groups[1];
    let sgn = if &groups[2] == "-" { -1 } else { 1 };
    let arg = sgn * groups[3].parse::<i32>().unwrap();

    match op {
        "nop" => Op::Nop(arg),
        "acc" => Op::Acc(arg),
        "jmp" => Op::Jmp(arg),
        _ => {
            panic!("Unknown op: {}", op);
        }
    }
}

fn read_program(path: &str) -> Vec<Op> {
    util::read_lines(path)
        .unwrap()
        .map(|line| line.unwrap())
        .map(|line| parse_instruction(&line))
        .collect()
}

/// Returns the final accumulator value or None if the program goes into an infinite loop.
fn run_program(ops: &Vec<Op>) -> Option<i32> {
    let mut line = 0usize;
    let mut acc = 0;
    let mut run_lines: HashSet<usize> = HashSet::new();
    // let mut n = 0;

    loop {
        // n += 1;
        // println!("{} {}: {:?} acc {}", n, line, &ops[line], acc);
        if run_lines.contains(&line) {
            // println!("{} {}: {:?} acc {}", n, line, &ops[line], acc);
            return None;
        } else if line == ops.len() {
            return Some(acc);
        }

        let mut next_line = line + 1;
        run_lines.insert(line);
        let op = &ops[line];
        match op {
            Op::Nop(_arg) => {}
            Op::Acc(arg) => {
                acc += arg;
            }
            Op::Jmp(arg) => {
                next_line = (line as i32 + arg) as usize;
            }
        }
        line = next_line;
    }
}

fn process_file(path: &str) {
    let program = read_program(path);
    for (i, op) in program.iter().enumerate() {
        let mut variation = program.to_vec();
        variation[i] = match op {
            Op::Nop(arg) => Op::Jmp(*arg),
            Op::Jmp(arg) => Op::Nop(*arg),
            Op::Acc(arg) => Op::Acc(*arg),
        };
        if let Some(acc) = run_program(&variation) {
            println!("swap: {}, acc: {}", i, acc);
            break;
        }
    }
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
        assert_eq!(parse_instruction("nop +0"), Op::Nop(0));
        assert_eq!(parse_instruction("jmp -4"), Op::Jmp(-4));
    }
}
