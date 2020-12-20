#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
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

fn parse_grid(lines: &[&str]) -> Vec<Vec<bool>> {
    lines.iter().map(|line| line.trim().chars().map(|c| c == '#').collect()).collect()
}

fn parse_tile(tile: &str) -> Tile {
    let mut lines = tile.lines();
    let title = lines.next().unwrap();
    let tile_cap = TILE_RE.captures(title).unwrap();
    let id: u64 = tile_cap[1].parse().unwrap();

    let px = parse_grid(&lines.collect_vec());
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

fn hashset(xs: &[u32]) -> HashSet<u32> {
    xs.iter().map(|&x| x).collect()
}

// TODO: move this into utils.rs
macro_rules! set(
    { $($key:expr),+ } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($key);
            )+
            m
        }
     };
);

macro_rules! map(
    { $($key:expr => $val:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $val);
            )+
            m
        }
     };
);

fn masks(tile: &Tile) -> HashSet<u32> {
    set!{tile.left, tile.right, tile.top, tile.bottom}
}

fn flipped_masks(tile: &Tile) -> HashSet<u32> {
    set!{
        flip_bits(tile.left, 10),
        flip_bits(tile.right, 10),
        flip_bits(tile.top, 10),
        flip_bits(tile.bottom, 10)
    }
}

fn possible_masks(tile: &Tile) -> HashSet<u32> {
    masks(tile).union(&flipped_masks(tile)).map(|x| *x).collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Identity,
    FlipVert,
    FlipHoriz,
    Rot90,
    Rot180,
    Rot270,
    FlipDiagTLBR,
    FlipDiagBLTR
}

const OPS: [Op; 8] = [
    Op::Identity,
    Op::FlipVert,
    Op::FlipHoriz,
    Op::Rot90,
    Op::Rot180,
    Op::Rot270,
    Op::FlipDiagTLBR,
    Op::FlipDiagBLTR
];

fn rot90(px: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let n = px.len();
    let mut out = ((0..n).map(|y| vec![false; n])).collect_vec();

    for (y, row) in px.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            // n = 10
            // (y, x)
            // (0, 0) -> (0, 9)
            // (0, 9) -> (9, 9)
            // (9, 0) -> (0, 0)
            // (9, 9) -> (9, 0)
            out[x][n - 1 - y] = *v;
        }
    }

    out
}

fn transform_tile(tile: &Tile, op: Op) -> Tile {
    let n = tile.px.len() as u32;
    let Tile {id, left, right, top, bottom, px: _} = *tile;
    match op {
        Op::Identity => tile.clone(),
        Op::FlipVert => Tile {
            id,
            top: bottom,
            bottom: top,
            left: flip_bits(left, n),
            right: flip_bits(right, n),
            px: tile.px.iter().rev().map(|row| row.clone()).collect_vec(),
        },
        Op::FlipHoriz => Tile {
            id,
            top: flip_bits(top, n),
            bottom: flip_bits(bottom, n),
            left: right,
            right: left,
            px: tile.px.iter().map(
                |row| row.iter().rev().map(|b| *b).collect_vec()
            ).collect_vec(),
        },
        Op::Rot90 => Tile {
            id,
            top: flip_bits(left, n),
            right: top,
            bottom: flip_bits(right, n),
            left: bottom,
            px: rot90(&tile.px),
        },
        Op::Rot180 => transform_tile(&transform_tile(tile, Op::Rot90), Op::Rot90),
        // TODO: I think this is a flip + rotate?
        Op::Rot270 => transform_tile(&transform_tile(&transform_tile(tile, Op::Rot90), Op::Rot90), Op::Rot90),
        // TODO: These could be implemented more efficiently
        Op::FlipDiagTLBR => transform_tile(&transform_tile(tile, Op::Rot90), Op::FlipHoriz),
        Op::FlipDiagBLTR => transform_tile(&transform_tile(tile, Op::Rot90), Op::FlipVert),
    }
}

fn index_tiles(tiles: &[Tile]) -> HashMap<u32, Vec<&Tile>> {
    let mut out: HashMap<u32, Vec<&Tile>> = HashMap::new();
    for tile in tiles.iter() {
        for mask in possible_masks(tile).iter() {
            (*out.entry(*mask).or_insert(vec![])).push(tile);
        }
    }
    out
}

fn possible_neighbors<'a>(tile: &Tile, index: &'a HashMap<u32, Vec<&Tile>>) -> Vec<&'a Tile> {
    let mut out: Vec<&Tile> = vec![];
    let mut ids = set!{tile.id};
    for mask in masks(tile) {
        for other in index.get(&mask).unwrap_or(&vec![]) {
            if !ids.contains(&other.id) {
                out.push(other);
                ids.insert(other.id);
            }
        }
    }
    out
}

fn add_to_right(left: &Tile, right: &Tile) -> Option<Op> {
    let n = left.px.len() as u32;
    let mask = left.right;
    let mask_flip = flip_bits(mask, n);
    if right.left == mask {
        return Some(Op::Identity);
    } else if right.left == mask_flip {
        return Some(Op::FlipVert);
    } else if right.bottom == mask {
        return Some(Op::Rot90);
    } else if right.bottom == mask_flip {
        return Some(Op::FlipDiagBLTR);
    } else if right.right == mask {
        return Some(Op::FlipHoriz);
    } else if right.right == mask_flip {
        return Some(Op::Rot180);
    } else if right.top == mask {
        return Some(Op::FlipDiagTLBR);
    } else if right.top == mask_flip {
        return Some(Op::Rot270);
    }

    None
}

fn add_to_bottom(top: &Tile, bottom: &Tile) -> Option<Op> {
    let n = top.px.len() as u32;
    let mask = top.bottom;
    let mask_flip = flip_bits(mask, n);
    if bottom.top == mask {
        return Some(Op::Identity);
    } else if bottom.top == mask_flip {
        return Some(Op::FlipHoriz);
    } else if bottom.left == mask {
        return Some(Op::FlipDiagTLBR);
    } else if bottom.left == mask_flip {
        return Some(Op::Rot90);
    } else if bottom.bottom == mask {
        return Some(Op::FlipVert);
    } else if bottom.bottom == mask_flip {
        return Some(Op::Rot180);
    } else if bottom.right == mask {
        return Some(Op::Rot270);
    } else if bottom.right == mask_flip {
        return Some(Op::FlipDiagBLTR);
    }

    None
}

fn print_grid(grid: &HashMap<(i32, i32), Tile>, n: i32) {
    for y in 0..n {
        println!("{}", (0..n).map(
            |x| grid.get(&(x, y)).map_or(String::from(""), |t| t.id.to_string())
        ).join(", "));
    }
}

fn fill_grid(tiles: &[Tile], top_left: &Tile, right: &Tile, below: &Tile, neighbors: HashMap<u64, Vec<&Tile>>) -> HashMap<(i32, i32), Tile> {
    let n = (tiles.len() as f64).sqrt() as i32;
    let mut used = set!{top_left.id, right.id, below.id};
    let id_to_tile = tiles.iter().map(|t| (t.id, t)).collect::<HashMap<u64, &Tile>>();

    // These three should be enough to orient the grid
    let mut grid = map! {
        (0, 0) => top_left.clone(),
        (1, 0) => right.clone(),
        (0, 1) => below.clone()
    };

    let neighbs = |id, u: &HashSet<u64>| {
        neighbors[&id].iter().filter(|&n| !u.contains(&n.id)).map(|x| *x).collect_vec()
    };

    // n = 3
    // diag = 4
    for diag in 2..(2*n - 1) {
        if diag < n {
            let bottom = grid.get(&(0, diag - 1)).unwrap();
            let bottoms = neighbs(bottom.id, &used).iter()
                .filter_map(
                    |t| add_to_bottom(bottom, t)
                        .and_then(|op| Some(transform_tile(t, op)))
                ).collect_vec();
            assert_eq!(1, bottoms.len());
            grid.insert((0, diag), bottoms[0].clone());

            let right = grid.get(&(diag - 1, 0)).unwrap();
            let rights = neighbs(right.id, &used).iter()
                .filter_map(
                    |t| add_to_right(right, t)
                        .and_then(|op| Some(transform_tile(t, op)))
                ).collect_vec();
            assert_eq!(1, rights.len());
            grid.insert((diag, 0), rights[0].clone());
        }

        for x in 0..n {
            let y = diag - x;
            if 0 <= y && y < n && !grid.contains_key(&(x, y)) {
                // Find the (unique) tile that can go at (x, y)
                let left = grid.get(&(x - 1, y)).unwrap();
                let above = grid.get(&(x, y - 1)).unwrap();
                let left_ids = neighbs(left.id, &used).iter().map(|t| t.id).collect::<HashSet<_>>();
                let above_ids = neighbs(above.id, &used).iter().map(|t| t.id).collect::<HashSet<_>>();
                let ids = left_ids.intersection(&above_ids).collect_vec();
                if ids.len() != 1 {
                    panic!("Impossible situation: ({}, {}) => {} candidates", x, y, ids.len());
                }
                let t = id_to_tile[ids[0]];
                if let Some(op) = add_to_right(left, t) {
                    grid.insert((x, y), transform_tile(t, op));
                    used.insert(t.id);
                }
            }
        }
    }

    grid
}

fn process_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let chunks = contents.split("\n\n").collect::<Vec<_>>();

    let tiles = chunks.iter().map(|chunk| parse_tile(chunk)).collect::<Vec<_>>();

    // let edges = tiles.iter().flat_map(|tile| vec![tile.left, tile.right, tile.top, tile.bottom]).collect::<HashSet<_>>();

    println!("# tiles: {}", tiles.len());
    // println!("# distinct edges: {}", edges.len());

    let mask_to_tiles = index_tiles(&tiles);
    let id_to_tile = tiles.iter().map(|t| (t.id, t)).collect::<HashMap<u64, &Tile>>();

    let mut neighbor_map = HashMap::new();
    let mut corners: Vec<&Tile> = vec![];
    for (i, tile) in tiles.iter().enumerate() {
        let neighbors = possible_neighbors(&tile, &mask_to_tiles).iter().map(|t| t.id).collect_vec();
        println!("{} {} -> {:?}", i, tile.id, neighbors);
        if neighbors.len() == 2 {
            corners.push(tile);
        }
        neighbor_map.insert(tile.id, neighbors.iter().map(|id| id_to_tile[id]).collect_vec());
    }

    println!("Corners: {:?}", corners.iter().map(|t| t.id).collect_vec());
    println!("Product: {}", corners.iter().map(|t| t.id).product::<u64>());

    assert_eq!(corners.len(), 4);
    let top_left = corners[0];
    let tln = possible_neighbors(top_left, &mask_to_tiles);
    assert_eq!(2, tln.len());
    // TODO: why can't I move out of this vector?
    let mut tln0 = tln[0].clone();
    let mut tln1 = tln[1].clone();
    let tl_tile = OPS.iter().find_map(|&op| {
        let t = transform_tile(top_left, op);
        let op0 = add_to_right(&t, &tln0);
        let op1 = add_to_bottom(&t, &tln1);
        if op0.is_some() && op1.is_some() {
            println!("top left: {}", t.id);
            println!("      op: {:?}", op);
            println!("   right: {} {:?}", tln0.id, op0);
            println!("  bottom: {} {:?}", tln1.id, op1);
            tln0 = transform_tile(&tln0, op0.unwrap());
            tln1 = transform_tile(&tln1, op1.unwrap());
            return Some(t);
        }
        return None;
    }).unwrap();

    let n = (tiles.len() as f64).sqrt() as i32;
    let grid = fill_grid(&tiles, &tl_tile, &tln0, &tln1, neighbor_map);
    print_grid(&grid, n);
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

    #[test]
    fn test_parse_grid() {
        assert_eq!(
            parse_grid(&r"..##.#
            ##..#.
            #...##
            ####.#
            ##.##.
            ##...#".lines().collect_vec()),
            vec![
                vec![false, false, true, true, false, true],
                vec![true, true, false, false, true, false],
                vec![true, false, false, false, true, true],
                vec![true, true, true, true, false, true],
                vec![true, true, false, true, true, false],
                vec![true, true, false, false, false, true],
            ]
        );
    }

    fn grid_to_str(px: &Vec<Vec<bool>>) -> String {
        px.iter().map(|row| row.iter().map(|c| if *c { '#' } else { '.' }).collect::<String>()).join("\n")
    }

    #[test]
    fn test_rot() {
        let px = parse_grid(
            &r"#.#
               #..
               ##.".lines().collect_vec());
        assert_eq!(grid_to_str(&px),
            r"#.#
              #..
              ##.".replace(" ", "")
        );
        assert_eq!(grid_to_str(&rot90(&px)),
            r"###
              #..
              ..#".replace(" ", "")
        );
    }

    #[test]
    fn test_transform_tile() {
        let tile = parse_tile(
            &r"Tile 123:
               #.#
               #..
               ##.");
        assert_eq!(tile.top, 5);
        assert_eq!(tile.left, 7);

        assert_eq!(transform_tile(&tile, Op::Identity), tile);
        assert_eq!(transform_tile(&tile, Op::FlipHoriz), parse_tile(
            &r"Tile 123:
               #.#
               ..#
               .##"
        ));
        assert_eq!(transform_tile(&tile, Op::FlipVert), parse_tile(
            &r"Tile 123:
               ##.
               #..
               #.#"
        ));
        assert_eq!(transform_tile(&tile, Op::Rot90), parse_tile(
            &r"Tile 123:
               ###
               #..
               ..#"
        ));
        assert_eq!(transform_tile(&tile, Op::FlipDiagTLBR), parse_tile(
            &r"Tile 123:
               ###
               ..#
               #.."
        ));
        assert_eq!(transform_tile(&tile, Op::FlipDiagBLTR), parse_tile(
            &r"Tile 123:
               ..#
               #..
               ###"
        ));
    }

    #[test]
    fn test_add_to_right() {
        let left = parse_tile(
            &r"Tile 123:
               #.#
               #..
               ##.");

        assert_eq!(add_to_right(
            &left,
            &parse_tile(
                &r"Tile 2:
                   ###
                   .#.
                   .##")), Some(Op::Identity));

        assert_eq!(add_to_right(
        &left,
        &parse_tile(
            &r"Tile 2:
                ...
                #..
                #..")), Some(Op::Rot90));

        assert_eq!(add_to_right(
            &left,
            &parse_tile(
                &r"Tile 2:
                    #..
                    ###
                    #.#")), Some(Op::FlipDiagTLBR));
    }

    #[test]
    fn test_add_to_bottom() {
        let top = parse_tile(
            &r"Tile 123:
               #.#
               #..
               ##.");

        assert_eq!(add_to_bottom(
            &top,
            &parse_tile(
                &r"Tile 2:
                   ##.
                   #..
                   #.#")), Some(Op::Identity));

        assert_eq!(add_to_bottom(
            &top,
            &parse_tile(
                &r"Tile 123:
                   #.#
                   #..
                   ##.")), Some(Op::FlipVert));
    }
}
