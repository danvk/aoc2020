// Each allergen is found in exactly one ingredient.
// Each ingredient contains zero or one allergen.
// Allergens aren't always marked.
// Even if an allergen isn't listed, the ingredient that contains that allergen could still be present

// 4             mxmxvkd      sbzzf sqjhc     (contains fish)
// 1       kfcds mxmxvkd nhms       sqjhc     (contains dairy, fish)
// 2 fvjkl       mxmxvkd      sbzzf       trh (contains dairy)
// 3 fvjkl                          sqjhc     (contains soy)

// fvjkl
// kfcds
// mxmxvkd
// nhms
// sbzzf
// sqjhc
// trh

// kfcds, nhms, sbzzf, or trh cannot contain an allergen

use aoc2020::util;

fn read_ints(path: &str) -> Vec<i32> {
  let mut out: Vec<i32> = Vec::new();
  let lines = util::read_lines(path).unwrap();

  for line in lines {
    let num = line.unwrap().parse::<i32>().unwrap();
    out.push(num);
  }

  out
}

fn main() {
  let nums = read_ints("inputs/day1.txt");
  println!("Read {} nums", nums.len());

  for num1 in nums.iter() {
    for num2 in nums.iter() {
      for num3 in nums.iter() {
        if num1 + num2 + num3 == 2020 {
          println!("{} * {} * {} = {}", num1, num2, num3, num1 * num2 * num3);
        }
      }
    }
  }
}
