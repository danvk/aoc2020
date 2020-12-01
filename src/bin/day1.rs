/*
const NUMS: [i32; 6] = [
  1721,
  979,
  366,
  299,
  675,
  1456
];
*/

use super::super::util;

fn main() {
  for num1 in nums.iter() {
    for num2 in nums.iter() {
      if num1 + num2 == 2020 {
        println!("{} * {} = {}", num1, num2, num1 * num2);
      }
    }
  }
}
