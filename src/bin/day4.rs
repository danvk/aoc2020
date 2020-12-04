use std::env;
use std::collections::HashMap;
use regex::Regex;
use aoc2020::util;

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
    let pass = Passport{
      byr: String::from(fields.get("byr").unwrap()),
      iyr: String::from(fields.get("iyr").unwrap()),
      eyr: String::from(fields.get("eyr").unwrap()),
      hgt: String::from(fields.get("hgt").unwrap()),
      hcl: String::from(fields.get("hcl").unwrap()),
      ecl: String::from(fields.get("ecl").unwrap()),
      pid: String::from(fields.get("pid").unwrap()),
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


fn validate_passport(pass: &Passport) -> bool {
  let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
  let ecl_re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
  let hgt_re = Regex::new(r"^(\d{2,3})(in|cm)$").unwrap();

  if pass.byr.as_str() < "1920" || pass.byr.as_str() > "2002" {
    println!("invalid byr: {}", pass.byr);
    return false;
  }
  if pass.iyr.as_str() < "2010" || pass.iyr.as_str() > "2020" {
    println!("invalid iyr: {}", pass.iyr);
    return false;
  }
  if pass.eyr.as_str() < "2020" || pass.eyr.as_str() > "2030" {
    println!("invalid eyr: {}", pass.eyr);
    return false;
  }
  if !hcl_re.is_match(&pass.hcl) {
    println!("invalid hcl: {}", pass.hcl);
    return false;
  }
  if !pid_re.is_match(&pass.pid) {
    println!("invalid pid: {}", pass.pid);
    return false;
  }
  if !ecl_re.is_match(&pass.ecl) {
    println!("invalid ecl: {}", pass.ecl);
    return false;
  }
  match hgt_re.captures(&pass.hgt) {
    None => { return false; }
    Some(caps) => {
      let val = caps[1].parse::<i32>().unwrap();
      let units = &caps[2];
      if units == "in" {
        //   If in, the number must be at least 59 and at most 76.
        if val < 59 || val > 76 {
          return false;
        }
      } else if units == "cm" {
        //   If cm, the number must be at least 150 and at most 193.
        if val < 150 || val > 193 {
          return false;
        }
      } else {
        return false;
      }
    }
  }

  true
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
