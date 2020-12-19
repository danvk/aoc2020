#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use itertools::Itertools;

#[derive(Debug)]
enum Rule {
    Literal(char),
    Pattern(Vec<Vec<i32>>),
}

impl Rule {
    /// Try to match the start of txt, returning the remainder.
    fn match_str<'a>(&self, txt: &'a str, rules: &HashMap<i32, Rule>) -> Option<&'a str> {
        // println!("{} match? {} Rule {:?}", &indent, txt, self);
        if txt.is_empty() {
            return None;
        }

        match self {
            Rule::Literal(c) => if txt.chars().next().unwrap_or(' ') == *c { Some(&txt[1..]) } else { None },
            Rule::Pattern(pats) => {
                for pat in pats {
                    let mut rest = txt;
                    let mut is_match = false;
                    for id in pat {
                        let rule = rules.get(id).unwrap();
                        if let Some(r) = rule.match_str(rest, rules) {
                            is_match = true;
                            rest = r;
                        } else {
                            is_match = false;
                            break;
                        }
                    }
                    if is_match {
                        // println!("{} match! {:?}", indent, pat);
                        return Some(rest);
                    }
                }

                None
            }
        }
    }
}

lazy_static! {
    // 3: 4 5 | 5 4
    // 4: "a"
    static ref LITERAL_RE: Regex = Regex::new(r#"^ *(\d+): "([a-z])"$"#).unwrap();
    static ref PATTERN_RE: Regex = Regex::new(r#"^ *(\d+): (.*)$"#).unwrap();
}

fn parse_rule(rule: &str) -> (i32, Rule) {
    if let Some(caps) = LITERAL_RE.captures(rule) {
        let id = caps[1].parse::<i32>().unwrap();
        let c = caps[2].parse::<char>().unwrap();
        return (id, Rule::Literal(c));
    }
    if let Some(caps) = PATTERN_RE.captures(rule) {
        let id = caps[1].parse::<i32>().unwrap();
        let rest = &caps[2];
        let pats = rest.split(" | ")
            .map(|pat| pat.split(' ')
                .map(|n| n.parse::<i32>().expect(&format!("failed to parse: {}", n)))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        return (id, Rule::Pattern(pats));
    }

    unreachable!("Rule: {}", rule)
}

fn parse_rules(rules: &str) -> HashMap<i32, Rule> {
    rules.split('\n').map(|rule| parse_rule(rule)).collect::<HashMap<_, _>>()
}

fn expand_rule(rule: &Rule, rules: &HashMap<i32, Rule>) -> HashSet<String> {
    let mut s = HashSet::new();
    match rule {
        Rule::Literal(c) => { s.insert(c.to_string()); },
        Rule::Pattern(pats) => {
            for pat in pats {
                let mut pieces = pat.iter().map(|i| expand_rule(&rules[i], rules)).collect::<Vec<_>>();
                let mut poss = pieces.remove(0);
                for piece in pieces {
                    let x = poss.iter().cartesian_product(piece.iter());
                    poss = x.map(|(a, b)| {
                        let mut s = a.to_owned();
                        s.push_str(b);
                        s
                    }).collect();
                }
                s.extend(poss);
            }
        },
    }
    s
}

fn match2(text: &str, starts: &HashSet<String>, ends: &HashSet<String>) -> bool {
    // println!("{}", text);
    let n = starts.iter().next().unwrap().len();
    for start in starts {
        assert_eq!(start.len(), n);
    }
    for end in ends {
        assert_eq!(end.len(), n);
        assert!(!starts.contains(end));
    }

    let mut num42 = 0;
    let mut rest: &str = text;
    while n <= rest.len() && starts.contains(&rest[..n]) {
        // println!(" 42: {}", &rest[..n]);
        rest = &rest[n..];
        num42 += 1;
    }

    let mut num31 = 0;
    while n <= rest.len() && ends.contains(&rest[..n]) {
        // println!(" 31: {}", &rest[..n]);
        rest = &rest[n..];
        num31 += 1;
    }

    // println!(" 42: {}, 31: {}, rest: {}", num42, num31, rest);
    rest == "" && num31 < num42 && num31 > 0 && num42 > 0
}

fn process_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let chunks = contents.split("\n\n").collect::<Vec<_>>();
    assert_eq!(2, chunks.len());

    let rules = parse_rules(&chunks[0]);
    let rule0 = &rules[&0];

    let rule42s = expand_rule(&rules[&42], &rules);
    let rule31s = expand_rule(&rules[&31], &rules);
    println!("42s ({}): {:?}", rule42s.len(), rule42s);
    println!("31s ({}): {:?}", rule31s.len(), rule31s);
    println!("len: {}", rule31s.iter().next().unwrap().len());

    let mut num_ok1 = 0;
    let mut num_ok2 = 0;
    for line in chunks[1].split('\n') {
        if line.is_empty() {
            continue;
        }
        let is_ok1 = rule0.match_str(line, &rules) == Some("");
        let is_ok2 = match2(line, &rule42s, &rule31s);
        // println!("{}: {}\n", is_ok2, line);
        num_ok1 += if is_ok1 { 1 } else { 0 };
        num_ok2 += if is_ok2 { 1 } else { 0 };
    }
    println!("Num OK (part 1): {}", num_ok1);
    println!("Num OK (part 2): {}", num_ok2);
    // not 68
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
    fn test0() {
        let rules = parse_rules(r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b""#);
        let rule0 = &rules[&0];
        println!("rules: {:?}", rules);
        // aab or aba
        assert_eq!(Some(""), rule0.match_str("aab", &rules));
        assert_eq!(Some(""), rule0.match_str("aba", &rules));
        assert_eq!(None, rule0.match_str("bab", &rules));
    }


    #[test]
    fn test1() {
        let rules = parse_rules(r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#);
        let rule0 = &rules[&0];
        println!("rules: {:?}", rules);
        // ababbb and abbbab match, but
        // bababa, aaabbb, and aaaabbb do not
        assert_eq!(Some(""), rule0.match_str("ababbb", &rules));
        assert_eq!(Some(""), rule0.match_str("abbbab", &rules));
        assert_eq!(None, rule0.match_str("bababa", &rules));
        assert_eq!(None, rule0.match_str("aaabbb", &rules));
        assert_eq!(None, rule0.match_str("aaaabbb", &rules));
    }

    use std::iter::FromIterator;

    #[test]
    fn test_expand() {
        let rules = parse_rules(r#"0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b""#);
        // TODO: find a better way to write HashSet<String> literals
        assert_eq!(expand_rule(&rules[&4], &rules), HashSet::from_iter(vec![String::from("a")]));
        assert_eq!(
            expand_rule(&rules[&3], &rules),
            HashSet::from_iter(vec![String::from("ab"), String::from("ba")])
        );
        assert_eq!(
            expand_rule(&rules[&2], &rules),
            HashSet::from_iter(vec![String::from("aa"), String::from("bb")])
        );
        assert_eq!(
            expand_rule(&rules[&1], &rules),
            HashSet::from_iter(vec![
                String::from("aaab"),
                String::from("aaba"),
                String::from("bbab"),
                String::from("bbba"),
                String::from("abaa"),
                String::from("abbb"),
                String::from("baaa"),
                String::from("babb"),
            ])
        );
    }
}
