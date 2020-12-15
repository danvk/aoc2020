use std::{collections::{HashMap, hash_map::Entry}, env};
use std::time::Instant;

fn play_game(start: &[i32], num_rounds: i32) -> i32 {
    let mut num_to_round: HashMap<i32, i32> = HashMap::new();
    let mut last_spoken = 0;
    for (n, &i) in start.iter().enumerate() {
        if n > 0 {
            num_to_round.insert(last_spoken, (n as i32) - 1);
        }
        last_spoken = i;
        // println!("{}: last_spoken={}, nums={:?}", 1 + n, last_spoken, num_to_round);
    }

    for i in (start.len() as i32)..num_rounds {
        let prev_last = last_spoken;
        match num_to_round.get(&last_spoken) {
            Some(&last) => {
                // println!("{}  {} last spoken at {}", i, last_spoken, last);
                last_spoken = i - last - 1;
            },
            None => {
                // println!("{}  {} never spoken before", i, last_spoken);
                last_spoken = 0;
            }
        }
        num_to_round.insert(prev_last, i - 1);
        // println!("{}: last_spoken={}, nums={:?}", 1 + i, last_spoken, num_to_round);
    }

    last_spoken
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Expected two arguments, got {}: {:?}", args.len(), args);
    }

    let nums = args[1].split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let rounds = args[2].parse::<i32>().unwrap();

    println!("nums: {:?}", nums);
    let last_spoken = play_game(&nums, rounds);
    println!("last spoken: {} after {} rounds", last_spoken, rounds);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

}
