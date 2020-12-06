#[macro_use]
extern crate lazy_static;
use aoc2020::util;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env;

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

fn process_passport(text: &str) -> Option<Passport> {
    // println!("Passport:\n{}\n---", text);

    let fields = text
        .split_whitespace()
        .flat_map(|kv| kv.split(':'))
        .tuples()
        .collect::<HashMap<_, _>>();

    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID)

    let ok = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|k| fields.contains_key(k));

    if ok {
        let pass = Passport {
            byr: String::from(*fields.get("byr").unwrap()),
            iyr: String::from(*fields.get("iyr").unwrap()),
            eyr: String::from(*fields.get("eyr").unwrap()),
            hgt: String::from(*fields.get("hgt").unwrap()),
            hcl: String::from(*fields.get("hcl").unwrap()),
            ecl: String::from(*fields.get("ecl").unwrap()),
            pid: String::from(*fields.get("pid").unwrap()),
        };
        if validate_passport(&pass) {
            Some(pass)
        } else {
            None
        }
    } else {
        None
    }
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
//   If cm, the number must be at least 150 and at most 193.
//   If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

const ECLS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

lazy_static! {
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref HGT_RE: Regex = Regex::new(r"^(\d{2,3})(in|cm)$").unwrap();
}

fn validate_passport(pass: &Passport) -> bool {
    let byr = pass.byr.as_str();
    let iyr = pass.iyr.as_str();
    let eyr = pass.eyr.as_str();
    if byr < "1920"
        || byr > "2002"
        || iyr < "2010"
        || iyr > "2020"
        || eyr < "2020"
        || eyr > "2030"
        || !HCL_RE.is_match(&pass.hcl)
        || !PID_RE.is_match(&pass.pid)
        || !ECLS.contains(&pass.ecl.as_str())
    {
        return false;
    }

    match HGT_RE.captures(&pass.hgt) {
        None => {
            return false;
        }
        Some(caps) => {
            let val = caps[1].parse::<i32>().unwrap();
            let units = &caps[2];
            if !((units == "cm" && (150..=193).contains(&val))
                || (units == "in" && (59..=76).contains(&val)))
            {
                return false;
            }
        }
    }

    true
}

/*
fn chunks(path: &str) -> Vec<String> {
  let x = util::read_lines(path).unwrap()
  .into_iter()
  .group_by(|line| line.unwrap() != "")
  .into_iter()
  .filter(|(k, _v)| *k)
  .map(|(k, v)| v.into_iter().map(|v| v.unwrap()).join(" "))
  .collect::<Vec<String>>();

  x
}
*/

fn process_file(path: &str) {
    let mut current = String::from("");
    let mut num_ok = 0;
    let lines = util::read_lines(path).unwrap();
    for line_in in lines {
        let line = line_in.unwrap();
        if line == "" {
            if process_passport(&current).is_some() {
                num_ok += 1;
            }
            current = String::from("");
        } else {
            current += &line;
            current += " ";
        }
    }
    if process_passport(&current).is_some() {
        num_ok += 1;
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
