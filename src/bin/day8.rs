#[macro_use]
extern crate lazy_static;
// use std::collections::HashMap;
use aoc2020::util;
use regex::Regex;
use std::{collections::HashSet, env};

// TODO:   Change Instruction.op to be more like a tagged union
// TODO: x Use match / case on op.op
// TODO: x Use if let in process_file

#[derive(PartialEq, Eq, Clone, Debug)]
struct Instruction {
    op: String,
    arg: i32,
}

lazy_static! {
    static ref INSTR_RE: Regex = Regex::new(r"^([a-z]+) ([-+])([0-9]+)$").unwrap();
}

fn parse_instruction(text: &str) -> Instruction {
    let groups = INSTR_RE.captures(text).unwrap();
    let op = String::from(&groups[1]);
    let sgn = if &groups[2] == "-" { -1 } else { 1 };
    let arg = sgn * groups[3].parse::<i32>().unwrap();

    Instruction { op, arg }
}

fn read_program(path: &str) -> Vec<Instruction> {
    util::read_lines(path)
        .unwrap()
        .map(|line| line.unwrap())
        .map(|line| parse_instruction(&line))
        .collect()
}

/// Returns the final accumulator value or None if the program goes into an infinite loop.
fn run_program(ops: &Vec<Instruction>) -> Option<i32> {
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
        match op.op.as_str() {
            "nop" => {}
            "acc" => {
                acc += op.arg;
            }
            "jmp" => {
                next_line = (line as i32 + op.arg) as usize;
            }
            _ => {
                panic!("Surprise op: {:?}", op);
            }
        }
        line = next_line;
    }
}

fn process_file(path: &str) {
    let program = read_program(path);
    for (i, op) in program.iter().enumerate() {
        let mut variation = program.to_vec();
        if op.op == "nop" {
            variation[i] = Instruction {
                op: String::from("jmp"),
                arg: op.arg,
            };
        } else if op.op == "jmp" {
            variation[i] = Instruction {
                op: String::from("nop"),
                arg: op.arg,
            };
        }
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
        assert_eq!(
            parse_instruction("nop +0"),
            Instruction {
                op: String::from("nop"),
                arg: 0
            }
        );

        assert_eq!(
            parse_instruction("jmp -4"),
            Instruction {
                op: String::from("jmp"),
                arg: -4
            }
        );
    }
}
