use std::env;
use std::collections::HashMap;
use regex::Regex;
use aoc2020::util;

// fn count_letters(txt: &str) -> HashMap<char, u32> {
//   let mut counts = HashMap::new();
//   for c in txt.chars() {
//     *counts.entry(c).or_insert(0) += 1;
//   }
//   counts
// }

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
  // let re: Regex = Regex::new(r"[ \n\r]+").unwrap();
  // println!("Passport:\n{}\n---", text);
  let mut fields: HashMap<String, String> = HashMap::new();
  for field in text.split_whitespace() {
    if field.chars().nth(3).unwrap() != ':' {
      panic!("Invalid field: {}", field);
    }
    fields.insert(field[0..3].to_string(), field[4..].to_string());
  }

  // byr (Birth Year)
  // iyr (Issue Year)
  // eyr (Expiration Year)
  // hgt (Height)
  // hcl (Hair Color)
  // ecl (Eye Color)
  // pid (Passport ID)
  // cid (Country ID)

  let ok = fields.contains_key("byr") &&
  fields.contains_key("iyr") &&
  fields.contains_key("eyr") &&
  fields.contains_key("hgt") &&
  fields.contains_key("hcl") &&
  fields.contains_key("ecl") &&
  fields.contains_key("pid");

  if ok {
    Some(Passport{
      byr: String::from(fields.get("byr").unwrap()),
      iyr: String::from(fields.get("iyr").unwrap()),
      eyr: String::from(fields.get("eyr").unwrap()),
      hgt: String::from(fields.get("hgt").unwrap()),
      hcl: String::from(fields.get("hcl").unwrap()),
      ecl: String::from(fields.get("ecl").unwrap()),
      pid: String::from(fields.get("pid").unwrap()),
    })
  } else {
    None
  }
}

fn process_file(path: &str) {
  let mut current = String::from("");
  let mut num_ok = 0;
  for line_in in util::read_lines(path).unwrap() {
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
