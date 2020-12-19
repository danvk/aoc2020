#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
enum Rule {
    Literal(char),
    Pattern(Vec<Vec<i32>>),
}

impl Rule {
    /// Try to match the start of txt, returning the remainder.
    fn match_str<'a>(&self, txt: &'a str, rules: &HashMap<i32, Rule>, indent: &str) -> Option<&'a str> {
        // println!("{} match? {} Rule {:?}", &indent, txt, self);
        if txt.is_empty() {
            return None;
        }
        // TODO: surely there is a more idiomatic way to do this
        let mut next_indent: String = String::from(" ");
        next_indent.push_str(&indent);
        match self {
            Rule::Literal(c) => if txt.chars().next().unwrap_or(' ') == *c { Some(&txt[1..]) } else { None },
            Rule::Pattern(pats) => {
                for pat in pats {
                    let mut rest = txt;
                    let mut is_match = false;
                    for id in pat {
                        let rule = rules.get(id).unwrap();
                        if let Some(r) = rule.match_str(rest, rules, &next_indent) {
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
    static ref LITERAL_RE: Regex = Regex::new(r#"^(\d+): "([a-z])"$"#).unwrap();
    static ref PATTERN_RE: Regex = Regex::new(r#"^(\d+): (.*)$"#).unwrap();
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

fn process_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let chunks = contents.split("\n\n").collect::<Vec<_>>();
    assert_eq!(2, chunks.len());

    let rules = parse_rules(&chunks[0]);
    println!("Rules: {:?}", rules);
    let rule0 = rules.get(&0).unwrap(); // why can't I do rules[0] here?

    let mut num_ok = 0;
    for line in chunks[1].split('\n') {
        if line.is_empty() {
            continue;
        }
        let is_ok = rule0.match_str(line, &rules, &"") == Some("");
        println!("{}: {}", is_ok, line);
        num_ok += if is_ok { 1 } else { 0 };
    }
    println!("Num OK: {}", num_ok);
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
        assert_eq!(Some(""), rule0.match_str("aab", &rules, ""));
        assert_eq!(Some(""), rule0.match_str("aba", &rules, ""));
        assert_eq!(None, rule0.match_str("bab", &rules, ""));
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
        // assert_eq!(Some(""), rule0.match_str("ababbb", &rules, ""));
        // assert_eq!(Some(""), rule0.match_str("abbbab", &rules));
        // assert_eq!(None, rule0.match_str("bababa", &rules));
        // assert_eq!(None, rule0.match_str("aaabbb", &rules));
        // assert_eq!(None, rule0.match_str("aaaabbb", &rules));
    }
}
