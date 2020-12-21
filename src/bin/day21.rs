#[macro_use]
extern crate lazy_static;
use aoc2020::util;
use regex::Regex;
use std::{collections::{HashMap, HashSet}, env};


lazy_static! {
  // mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
  static ref RECIPE_RE: Regex = Regex::new(r#"^ *([a-z ]+) \(contains (.*)\)$"#).unwrap();
}

fn parse_recipe(recipe: &str) -> (HashSet<String>, HashSet<String>) {
  let caps = RECIPE_RE.captures(recipe).unwrap();
  (
    caps[1].split(" ").map(|x| String::from(x)).collect(),
    caps[2].split(", ").map(|x| String::from(x)).collect()
  )
}

fn process_file(path: &str) {
    let recipes = util::read_lines(path).unwrap().map(|line| parse_recipe(&line.unwrap())).collect::<Vec<_>>();

    let mut all_ingredients = HashSet::new();
    let mut all_allergens = HashSet::new();
    for (ings, alls) in recipes.iter() {
      all_ingredients.extend(ings.iter());
      all_allergens.extend(alls.iter());
    }
    let ingredients = all_ingredients.iter().map(|i| *i).collect::<Vec<_>>();
    let allergens = all_allergens.iter().map(|a| *a).collect::<Vec<_>>();

    println!("{} Ingredients, {} allergens", ingredients.len(), allergens.len());

    let mut answer = 0;
    for &ing in ingredients.iter() {
      let mut excluded = HashSet::new();
      let mut num_app = 0;
      for (ings, alls) in recipes.iter() {
        if !ings.contains(ing) {
          excluded.extend(alls.iter());
        } else {
          num_app += 1;
        }
      }
      if excluded.len() == allergens.len() {
        println!("Excluded {}", ing);
        answer += num_app;
      }
    }

    println!("Answer: {}", answer);
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

}
