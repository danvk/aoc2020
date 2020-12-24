use std::{collections::HashMap, fmt, time::Instant};
use std::env;
use itertools::Itertools;

// 0   0 1 2 3 4
// 1  0 1 2 3 4
// 2   0 1 2 3 4
// 3  0 1 2 3 4

// east, southeast, southwest, west, northwest, and northeast.
// e, se, sw, w, nw, and ne

#[derive(Debug, PartialEq)]
enum HexDir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

const DIRS: [HexDir; 6] = [HexDir::E, HexDir::SE, HexDir::SW, HexDir::W, HexDir::NW, HexDir::NE];

fn hexmove(pos: &(i32, i32), dir: &HexDir) -> (i32, i32) {
    use HexDir::*;
    let &(x, y) = pos;
    match dir {
        E => (x + 1, y),
        W => (x - 1, y),
        // TODO: is this clearer with guards on the matches?
        NE => (if y.abs() % 2 == 1 { x } else { x + 1 }, y - 1 ),
        NW => (if y.abs() % 2 == 1 { x - 1 } else { x }, y - 1 ),
        SE => (if y.abs() % 2 == 1 { x } else { x + 1 }, y + 1 ),
        SW => (if y.abs() % 2 == 1 { x - 1 } else { x }, y + 1 ),
    }
}

fn hexmoves(moves: &[HexDir]) -> (i32, i32) {
    // let mut pos = (0, 0);
    // for dir in moves {
    //     pos = hexmove(pos, dir);
    //     println!("{:?} -> {:?}", dir, pos);
    // }
    moves.iter().fold((0, 0), |pos, dir| hexmove(&pos, dir))
    // pos
}

fn parse_line(line: &str) -> Vec<HexDir> {
    let cs = line.chars().collect_vec();
    let mut r = Vec::with_capacity(cs.len());
    let mut i = 0usize;
    while i < cs.len() {
        let c = cs[i];
        i += 1;
        if c == 'e' {
            r.push(HexDir::E);
        } else if c == 'w' {
            r.push(HexDir::W);
        } else if c == 'n' || c == 's' {
            let c1 = cs[i];
            r.push(match (c, c1) {
                ('n', 'e') => HexDir::NE,
                ('n', 'w') => HexDir::NW,
                ('s', 'e') => HexDir::SE,
                ('s', 'w') => HexDir::SW,
                _ => unreachable!(format!("{}, {}", c, c1)),
            });
            i += 1;
        } else {
            unreachable!();
        }
    }
    r
}

// Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.

type TileFloor = HashMap<(i32, i32), bool>;
// false = white
// true = black

fn remove_false(floor: &mut TileFloor) -> () {
    let whites = floor.iter().filter(|(_, v)| **v).map(|(k, _)| *k).collect_vec();
    for k in whites {
        floor.remove(&k);
    }
}

fn neighbors(pos: &(i32, i32)) -> Vec<(i32, i32)> {
    DIRS.iter().map(|d| hexmove(pos, d)).collect_vec()
}

fn num_neighbors(floor: &TileFloor, pos: &(i32, i32)) -> usize {
    DIRS.iter().map(|d| hexmove(pos, d)).filter(|p| *floor.get(p).unwrap_or(&false)).count()
}

fn next_day(floor: TileFloor) -> TileFloor {
    let mut next: TileFloor = HashMap::new();
    for (pos, &v) in floor.iter() {
        if v {
            // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
            let nn = num_neighbors(&floor, pos);
            if nn == 2 || nn > 2 {
                next.insert(*pos, true);
            } else {
                next.insert(*pos, false);
            }

            for n in neighbors(pos) {
                if !*floor.get(&n).unwrap_or(&false) && !next.contains_key(&n) {
                    // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
                    next.insert(*pos, num_neighbors(&floor, &n) == 2);
                }
            }
        }
    }

    next
}

fn num_black(floor: &TileFloor) -> usize {
    floor.values().filter(|&v| *v).count()
}

fn process_file(path: &str, num_days: usize) {
    let mut tiles: TileFloor = HashMap::new();
    for line in aoc2020::util::read_lines(path).unwrap().map(|line| line.unwrap()) {
        let m = parse_line(&line);
        let (x, y) = hexmoves(&m);
        let old_tile = *tiles.get(&(x, y)).unwrap_or(&false);
        tiles.insert((x, y), !old_tile);
    }

    // println!("Grid: {:?}", tiles);
    println!("num black: {}", num_black(&tiles));

    for day in 1..=num_days {
        tiles = next_day(tiles);
        println!("Day {}: {}", day, num_black(&tiles));
    }
}

fn main() {
    let args = env::args().collect_vec();
    if args.len() != 3 {
        panic!("Expected two arguments, got {}: {:?}", args.len(), args);
    }

    process_file(&args[1], args[2].parse::<usize>().unwrap());
}


#[cfg(test)]
mod tests {
    use aoc2020::map;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_line() {
        use HexDir::*;
        assert_eq!(parse_line("seswneswswsenwwnwse"), vec![SE, SW, NE, SW, SW, SE, NW, W, NW, SE]);
        assert_eq!(parse_line("nwwswee"), vec![NW, W, SW, E, E]);
    }

    #[test]
    fn test_move() {
        assert_eq!((1, 1), hexmoves(&parse_line("esew")));
        assert_eq!((0, 0), hexmoves(&parse_line("nwwswee")));
    }

    // [HexDir::E, HexDir::SE, HexDir::SW, HexDir::W, HexDir::NW, HexDir::NE];
    #[test]
    fn test_neighbors() {
        assert_eq!(neighbors(&(0, 0)), vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, -1),
        ]);
    }

    #[test]
    fn test_num_neighbors() {
        assert_eq!(num_neighbors(&map!{
            (1, 0) => true,
            (1, 1) => true,
            (0, 1) => true,
            (-1, 0) => true,
            (0, -1) => true,
            (1, -1) => true
        }, &(0, 0)), 6);

        assert_eq!(num_neighbors(&map!{
            (1, 0) => true,
            (1, 1) => true,
            (1, -1) => true
        }, &(0, 0)), 3);

        assert_eq!(num_neighbors(&map!{
            (1, 0) => true,
            (1, 1) => true,
            (0, 1) => true,
            (0, -1) => false,
            (1, -1) => true
        }, &(0, 0)), 4);
    }

//  y NW, W, SW, E, E
// -1 -1 0 1 2 3 4
//  0  -1 0 1 2 3 4
//  1 -1 0 1 2 3 4
//  2  -1 0 1 2 3 4
//  3 -1 0 1 2 3 4
}

// ideas:
// - use a better structure (vecdeque? linked list?)
// - figure out a math-y way to do it
