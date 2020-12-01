/*
const NUMS: [i32; 6] = [

];
*/

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
      if num1 + num2 == 2020 {
        println!("{} * {} = {}", num1, num2, num1 * num2);
      }
    }
  }
}
