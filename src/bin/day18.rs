#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use aoc2020::util;
use std::{env, time::Instant};

use pest::{Parser, iterators::{Pair, Pairs}, prec_climber::PrecClimber};
use pest::prec_climber::{Assoc,Operator};

#[derive(Parser)]
#[grammar = "day18.pest"]
struct ExprParser;

/*
enum Op {
    MUL,
    ADD
}
*/

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(multiply, Left),
            Operator::new(add, Left),
        ])
    };
}

// TODO: read a little more about how this works
// TODO: why do I need to trim the whitespace from my digits?
fn eval(expression: Pairs<Rule>) -> i64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::number => pair.as_str().trim().parse::<i64>().unwrap(),
            Rule::expr => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
            Rule::add      => lhs + rhs,
            Rule::multiply => lhs * rhs,
            _ => unreachable!(),
        },
    )
}

/*
fn evaluate_expr(expr: Pair<Rule>) -> i64 {
    let mut last_op: Option<Op> = None;
    let mut tally = 0;
    for term in expr.into_inner() {
        match term.as_rule() {
            Rule::number => {
                let num = term.as_str().trim().parse::<i64>().expect(&format!("Failed to parse '{}'", term.as_str()));
                match last_op {
                    None => tally = num,
                    Some(Op::ADD) => tally += num,
                    Some(Op::MUL) => tally *= num,
                }
                last_op = None;
            },
            Rule::add => {
                assert!(last_op.is_none());
                last_op = Some(Op::ADD);
            },
            Rule::multiply => {
                assert!(last_op.is_none());
                last_op = Some(Op::MUL);
            },
            Rule::expr => {
                let num = evaluate_expr(term);
                // TODO: merge w/ number arm
                match last_op {
                    None => tally = num,
                    Some(Op::ADD) => tally += num,
                    Some(Op::MUL) => tally *= num,
                }
                last_op = None;
            },
            _ => {
                println!("term: {:?}", term);
                unreachable!("Unimplemented term");
            }
        }
    }

    tally
}
*/


fn evaluate(text: &str) -> i64 {
    let expr = ExprParser::parse(Rule::calculation, text)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    // expr.into_inner();
    // println!("expr: {:?}", expr);
    eval(expr.into_inner())
}

fn process_file(path: &str) {
    let tally = util::read_lines(path).unwrap()
        .map(|line| evaluate(&line.unwrap()))
        .sum::<i64>();
    println!("Total: {}", tally);
}

// 2129 = too low
// 2130 = too low
// 2164 = correct

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    let now = Instant::now();
    process_file(&args[1]);
    println!("Done in {} ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test0() {
        assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6"), 51);
    }

    #[test]
    fn test1() {
        assert_eq!(evaluate("2 * 3 + (4 * 5)"), 46);
    }

    #[test]
    fn test2() {
        assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    }

    #[test]
    fn test3() {
        assert_eq!(evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    }

    #[test]
    fn test4() {
        assert_eq!(evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    }
}
