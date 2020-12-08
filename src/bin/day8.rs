#[macro_use]
extern crate lazy_static;
// use std::collections::HashMap;
use aoc2020::util;
use regex::Regex;
use std::{collections::HashSet, env};

#[derive(PartialEq, Eq, Debug)]
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

fn process_file(path: &str) {
    let program: Vec<Instruction> = util::read_lines(path)
        .unwrap()
        .map(|line| line.unwrap())
        .map(|line| parse_instruction(&line))
        .collect();

    let mut line = 0usize;
    let mut acc = 0;
    let mut run_lines: HashSet<usize> = HashSet::new();
    let mut n = 0;

    loop {
        n += 1;
        println!("{} {}: {:?} acc {}", n, line, &program[line], acc);
        if run_lines.contains(&line) {
            println!("line: {} accumulator: {}", line, acc);
            break;
        }

        run_lines.insert(line);
        let op = &program[line];
        if op.op == "nop" {
            line += 1;
        } else if op.op == "acc" {
            acc += op.arg;
            line += 1;
        } else if op.op == "jmp" {
            line = (line as i32 + op.arg) as usize;
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
