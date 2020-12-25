use std::env;
use itertools::Itertools;

fn transform(sub: u64, v: u64) -> u64 {
  (v * sub) % 20201227
}

fn find_loop_size(target: u64) -> usize {
  let sub = 7u64;
  let mut v = 1u64;
  let mut i = 0usize;
  while v != target {
    v = transform(sub, v);
    i += 1;
  }
  i
}

fn forward(v: u64, sub: u64, loop_size: usize) -> u64 {
  let mut v = v;
  for _ in 0..loop_size {
    v = transform(sub, v);
    // println!("{}: {}", i, v);
  }
  v
}

fn main() {
  let args = env::args().collect_vec();
  if args.len() < 2 {
      panic!("Expected 2+ arguments, got {}: {:?}", args.len(), args);
  }

  let v0 = args[1].parse::<u64>().unwrap();
  let v1 = args[2].parse::<u64>().unwrap();

  let loop_size = find_loop_size(v0);
  let v = forward(1, v1, loop_size);
  println!("{}", v);

  // for num_str in &args[1..] {
  //   let num = num_str.parse::<u64>().unwrap();
  //   // let loop_size = find_loop_size(num);
  //   println!("{}: {}", num, find_loop_size(num));
  // }
}

