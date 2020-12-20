#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
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

fn flip_bits(bits: u32, n: u32) -> u32 {
    (0..n).map(|i| ((bits & (1 << i)) >> i) << (n - 1 - i)).sum()
}

fn parse_tile(tile: &str) -> Tile {
    let mut lines = tile.lines();
    let title = lines.next().unwrap();
    let tile_cap = TILE_RE.captures(title).unwrap();
    let id: u64 = tile_cap[1].parse().unwrap();

    let px: Vec<Vec<_>> = lines.map(|line| line.trim().chars().map(|c| c == '#').collect()).collect();
    let top = to_mask(&px[0]);
    let bottom = to_mask(px.last().unwrap());
    let left = to_mask(&px.iter().map(|row| row[0]).collect::<Vec<_>>());
    let right = to_mask(&px.iter().map(|row| *row.last().unwrap()).collect::<Vec<_>>());

    Tile {
        id,
        px,
        left,
        right,
        top,
        bottom,
    }
}

enum Op {
    FlipVert,
    FlipHoriz,
    Rot90,
    Rot180,
    Rot270,
}

/*
fn transform(tile: &Tile, op: &Op) -> Tile {
    match op {

    }
}
*/

fn process_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let chunks = contents.split("\n\n").collect::<Vec<_>>();

    let tiles = chunks.iter().map(|chunk| parse_tile(chunk)).collect::<Vec<_>>();

    let edges = tiles.iter().flat_map(|tile| vec![tile.left, tile.right, tile.top, tile.bottom]).collect::<HashSet<_>>();

    println!("# tiles: {}", tiles.len());
    println!("# distinct edges: {}", edges.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    process_file(&args[1]);

    // Sample: 9 tiles, 27 distinct edges
    // Input: 144 tiles, 446 distinct edges
    // Tiles are 10x10
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_tile() {
        let tile = parse_tile(r#"Tile 2311:
        ..##.#
        ##..#.
        #...##
        ####.#
        ##.##.
        ##...#"#);
        assert_eq!(tile, Tile {
            id: 2311,
            px: vec![
                vec![false, false, true, true, false, true],
                vec![true, true, false, false, true, false],
                vec![true, false, false, false, true, true],
                vec![true, true, true, true, false, true],
                vec![true, true, false, true, true, false],
                vec![true, true, false, false, false, true],
            ],
            top: 1 + 4 + 8,
            bottom: 1 + 16 + 32,
            left: 1 + 2 + 4 + 8 + 16,
            right: 1 + 4 + 8 + 32,
        });
    }

    #[test]
    fn test_flip_bits() {
        // assert_eq!(flip_bits(0b1, 1), vec![0b1]);
        // assert_eq!(flip_bits(0b10, 2), vec![0, 0b01]);
        // assert_eq!(flip_bits(0b100, 3), vec![0, 0, 1]);
        // assert_eq!(flip_bits(0b110, 3), 0b011);
        assert_eq!(flip_bits(0b1000, 4), 0b0001);
        assert_eq!(flip_bits(0b1010, 4), 0b0101);
    }
}
