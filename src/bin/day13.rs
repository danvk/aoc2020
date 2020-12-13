use aoc2020::util;
use std::{env, fmt};

fn process_file(path: &str) {
    let mut lines = util::read_lines(path).unwrap();
    let t0 = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    let primes = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .filter(|&x| x != "x")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    println!("t0: {}", t0);
    println!("primes: {:?}", primes);

    let (wait, p) = primes.iter().map(|&p| ((p - (t0 % p)), p)).min().unwrap();
    println!("Answer (part 1): {} * {} = {}", wait, p, wait * p);
    /*
    for p in primes {
        let last = p * (t0 / p);
        let wait = p - (t0 % p);
        println!("{}, wait {} * {} -> {}", last, wait, p, wait * p);
    }
    */
}

// 2129 = too low
// 2130 = too low
// 2164 = correct

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
