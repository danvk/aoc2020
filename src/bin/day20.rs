#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use itertools::Itertools;

#[derive(Debug)]
struct Tile {
    id: u64,
    px: Vec<Vec<bool>>,
    // bitmasks, left and top = larger bits
    top: u32,
    left: u32,
    bottom: u32,
    right: u32,
}

// Tile 3079:
// #.#.#####.

lazy_static! {
    static ref TILE_RE: Regex = Regex::new(r#"^ *Tile (\d+):$"#).unwrap();
}

fn to_mask(bits: &[bool]) -> u32 {
    bits.iter().fold(0, |acc, x| 2 * acc + if *x { 1 } else { 0 })
}

fn parse_tile(tile: &str) -> Tile {
    let mut lines = tile.lines();
    let title = lines.next().unwrap();
    let tile_cap = TILE_RE.captures(title).unwrap();
    let id: u64 = tile_cap[1].parse().unwrap();

    let px = lines.map(|line| line.chars().map(|c| c == '#').collect()).collect();

    Tile {
        id,
        px,
        left: 0,
        right: 0,
        top: to_mask(&px[0]),
        bottom: 0,
    }
}

fn process_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let chunks = contents.split("\n\n").collect::<Vec<_>>();

    let tiles = chunks.iter().map(|chunk| parse_tile(chunk)).collect::<Vec<_>>();

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
