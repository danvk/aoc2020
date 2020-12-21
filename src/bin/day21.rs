#[macro_use]
extern crate lazy_static;
use aoc2020::util;
use regex::Regex;
use std::{collections::HashSet, env};


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

fn find_unsafe_ingredients<'a>(
  recipes: &Vec<(HashSet<String>, HashSet<String>)>,
  ingredients: &Vec<&'a String>,
  allergens: &Vec<&String>
) -> Vec<&'a String> {
  let mut answer = 0;
  let mut unsafe_ingredients = vec![];
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
      answer += num_app;
    } else {
      unsafe_ingredients.push(ing);
    }
  }

  println!("Answer (part 1): {}", answer);
  unsafe_ingredients
}

fn find_one_allergen(
  recipes: &Vec<(HashSet<String>, HashSet<String>)>,
  ingredients: &Vec<&String>,
  allergens: &Vec<&String>
) -> Option<(String, String)> {
  for &ing in ingredients.iter() {
    let mut excluded = HashSet::new();
    for (ings, alls) in recipes.iter() {
      if !ings.contains(ing) {
        excluded.extend(alls.iter());
      }
      // if alls.len() == 1 {
      //   // A recipe like X (contains Y) means that X contains Y.
      //   return Some((String::from(ing), String::from(alls.iter().next().unwrap())));
      // }
    }
    if excluded.len() == allergens.len() - 1 {
      let the_one = *allergens.iter().filter(|a| !excluded.contains(*a)).next().unwrap();
      return Some((String::from(ing), String::from(the_one)));
    }
  }

  None
}

fn filter_recipes(
  recipes: &Vec<(HashSet<String>, HashSet<String>)>,
  ingredients: &Vec<&String>,
  allergens: &Vec<&String>
) -> Vec<(HashSet<String>, HashSet<String>)> {
  let ing_set = ingredients.iter().map(|i| String::from(*i)).collect::<HashSet<_>>();
  let all_set = allergens.iter().map(|i| String::from(*i)).collect::<HashSet<_>>();

  // TODO: lots of copying here that feels unnecessary
  recipes.iter().map(
    |(ings, alls)| (
      ings.intersection(&ing_set).map(|x| String::from(x)).collect::<HashSet<_>>(),
      alls.intersection(&all_set).map(|x| String::from(x)).collect::<HashSet<_>>(),
    )
  ).collect::<Vec<_>>()
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
    let mut allergens = all_allergens.iter().map(|a| *a).collect::<Vec<_>>();

    println!("{} Ingredients, {} allergens", ingredients.len(), allergens.len());
    let mut ingredients = find_unsafe_ingredients(&recipes, &ingredients, &allergens);
    let mut recipes = filter_recipes(&recipes, &ingredients, &allergens);
    println!("Remaining recipes:");
    for (ings, alls) in &recipes {
      println!(" {:?} -> {:?}", ings, alls);
    }

    println!("Unsafe ingredients: {}", ingredients.len());
    println!("Allergens: {}", allergens.len());

    let mut mapping = vec![];
    while !allergens.is_empty() {
      let x = find_one_allergen(&recipes, &ingredients, &allergens);
      if let Some((ing, all)) = x {
        println!("{} = {}", all, ing);
        mapping.push((String::from(&all), String::from(&ing)));
        // TODO: this seems like a lot of fuss to remove an element from a vector
        ingredients.remove(ingredients.iter().position(|&x| *x == ing).unwrap());
        allergens.remove(allergens.iter().position(|&x| *x == all).unwrap());
        recipes = filter_recipes(&recipes, &ingredients, &allergens);
      } else {
        println!("--");
        for (ings, alls) in &recipes {
          println!(" {:?} -> {:?}", ings, alls);
        }
        panic!("Unable to find an allergen");
      }
    }

    mapping.sort();
    println!("mapping: {:?}", mapping);
    let ings = mapping.iter().map(|(_a, b)| String::from(b)).collect::<Vec<_>>();
    println!("answer: {}", ings.join(","));
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
