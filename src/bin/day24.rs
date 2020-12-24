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

fn hexmove(pos: (i32, i32), dir: &HexDir) -> (i32, i32) {
    use HexDir::*;
    let (x, y) = pos;
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
    moves.iter().fold((0, 0), |pos, dir| hexmove(pos, dir))
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

fn process_file(path: &str) {
    let mut tiles: HashMap<(i32, i32), bool> = HashMap::new();
    for line in aoc2020::util::read_lines(path).unwrap().map(|line| line.unwrap()) {
        let m = parse_line(&line);
        let (x, y) = hexmoves(&m);
        let old_tile = *tiles.get(&(x, y)).unwrap_or(&false);
        tiles.insert((x, y), !old_tile);
    }

    println!("Grid: {:?}", tiles);
    println!("num black: {}", tiles.values().filter(|&v| *v).count());
}

fn main() {
    let args = env::args().collect_vec();
    if args.len() != 2 {
        panic!("Expected two argument, got {}: {:?}", args.len(), args);
    }

    process_file(&args[1]);
}


#[cfg(test)]
mod tests {
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

//    NW, W, SW, E, E
// -1 -1 0 1 2 3 4
//  0  -1 0 1 2 3 4
//  1  -1 0 1 2 3 4
//  2 -1 0 1 2 3 4
//  3  -1 0 1 2 3 4
}

// ideas:
// - use a better structure (vecdeque? linked list?)
// - figure out a math-y way to do it
