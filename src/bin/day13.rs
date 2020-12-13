use aoc2020::util;
use std::{env};

/// Find the first n such that:
///   n = t1 (mod p1)
///   n = t2 (mod p2)
///
/// Subsequent ns will be n = n + k*p1*p2
fn first_congruence(p1: u64, t1: u64, p2: u64, t2: u64) -> u64 {
    if p1 < p2 {
        return first_congruence(p2, t2, p1, t1);
    }
    for k1 in 1..=p2 {
        let n = k1 * p1 + t1;
        if n % p2 == t2 {
            return n;
        }
    }
    panic!("Unable to find congruence: {} {}", p1, p2);
}

fn process_file(path: &str) {
    let mut lines = util::read_lines(path).unwrap();
    let t0 = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
    let mut pt = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(t, x)| (x.parse::<u64>().unwrap(), t as u64))
        .map(|(p, t)| (p, (p - (t % p)) % p))
        .collect::<Vec<_>>();
    // pt.sort();
    pt = pt.into_iter().rev().collect();

    println!("t0: {}", t0);
    println!("primes: {:?}", pt);

    let (mut p, mut t) = pt[0];
    assert!(t != 0);
    // let mut p0 = 0;
    for &(pi, ti) in &pt[1..] {
        // if ti == 0 {
        //     p0 = pi;
        //     continue;
        // }
        t = first_congruence(p, t, pi, ti);
        p *= pi;
    }
    println!("{} mod {}", t, p);

    // for w in tp.windows(2) {
    //     // Keep getting "refutable pattern in binding" errors here:
    //     // let &[(p1, t1), (p2, t2)] = w;
    //     let (p1, t1) = w[0];
    //     let (p2, t2) = w[1];
    //     println!("base ({}, {}) = {}", p1, p2, first_congruence(p1, t1 as u64, p2, t2 as u64));
    // }

    // let n = first_congruence(59 * 31, 592, 19 * 13, 118);
    // println!("fc 592, 118 = {}", n);
    // println!("fc 7 next = {}", first_congruence(7, 0, 59 * 31 * 19 * 13, n))
    // primes: [(59, 2), (31, 3), (19, 4), (13, 1), (7, 0)]
    // base (59, 31) = 592
    // base (31, 19) = 251
    // base (19, 13) = 118
    // base (13, 7) = 14
    // 181663

    // let (wait, p) = primes.iter().map(|&p| ((p - (t0 % p)), p)).min().unwrap();
    // println!("Answer (part 1): {} * {} = {}", wait, p, wait * p);

    // t = 0 (mod p1)
    // t = 1 (mod p2)
    // -> t = 0 (mod p1 * p2)
    // -> p1 * t = p1 (mod p2)

    // Find the first one, then subsequent ones will be by adding p1 * p2
    // 11 * 59 + 2 = 651 = 1 (mod 13)
    // 24 * 59 + 2 = 1418 = 1 (mod 13)
    // 37 * 59 + 2 = 2185 = 1 (mod 13)
    // 50 * 59 + 2 = 2952 = 1 (mod 13)
    // 63 * 59 + 2 = 3719 = 1 (mod 13)
    // 76 * 59 + 2 = 4486 = 1 (mod 13)
    // 89 * 59 + 2 = 5253 = 1 (mod 13)

    // t = 0 (mod 7)
    // t = 1 (mod 13)
    // t = 2 (mod 59)
    // t = 3 (mod 31)
    // t = 4 (mod 19)

    // t = 59k2 + 2
    // 7k1 = 59k2 + 2
    //

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

    #[test]
    fn test_find_congruence() {
        // assert_eq!(first_congruence(59, 2, 13, 1), 651);
        assert_eq!(first_congruence(41, 101, 977, 60), 0);
    }
}
