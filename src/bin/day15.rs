use std::{collections::HashMap, env};
use std::time::Instant;
// use rustc_hash::FxHashMap;

fn play_game(start: &[i32], num_rounds: i32) -> i32 {
    // let mut num_to_round: HashMap<i32, i32> = HashMap::new();
    // let mut num_to_round: FxHashMap<i32, i32> = FxHashMap::default();
    let mut num_to_round = vec![-1; num_rounds as usize];
    let mut last_spoken: i32 = 0;
    for (n, &i) in start.iter().enumerate() {
        if n > 0 {
            num_to_round[last_spoken as usize] = (n as i32) - 1;
        }
        last_spoken = i;
        // println!("{}: last_spoken={}, nums={:?}", 1 + n, last_spoken, num_to_round);
    }

    for i in (start.len() as i32)..num_rounds {
        // TODO: is it possible to do the insert immediately after the lookup?
        //       the borrow checker seems unhappy with that. Can I copy the option?
        let last = num_to_round[last_spoken as usize];
        num_to_round[last_spoken as usize] = i - 1;
        if last >= 0 {
            // println!("{}  {} last spoken at {}", i, last_spoken, last);
            last_spoken = i - last - 1;
        } else {
            // println!("{}  {} never spoken before", i, last_spoken);
            last_spoken = 0;
        }
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
    let start = Instant::now();
    let last_spoken = play_game(&nums, rounds);
    println!("last spoken: {} after {} rounds ({} ms)", last_spoken, rounds, start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

}
