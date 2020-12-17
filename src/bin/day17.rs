use aoc2020::util;
use std::{collections::HashMap, env, fmt};
use itertools::Itertools;


fn parse_char(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!("Invalid cell: {}", c),
    }
}

type Grid = HashMap<(i32, i32, i32), bool>;

fn parse_grid(path: &str) -> Grid {
    let mut grid: Grid = HashMap::new();
    for (y, line) in util::read_lines(path).unwrap().enumerate() {
        for (x, c) in line.unwrap().char_indices() {
            grid.insert((x as i32, y as i32, 0), parse_char(c));
        }
    }
    grid
}

const DS: [(i32, i32, i32); 26] = [
    (-1, -1, -1), (0, -1, -1), (1, -1, -1), (-1, 0, -1), (0, 0, -1), (1, 0, -1), (-1, 1, -1), (0, 1, -1), (1, 1, -1),
    (-1, -1, 0), (0, -1, 0), (1, -1, 0), (-1, 0, 0), (1, 0, 0), (-1, 1, 0), (0, 1, 0), (1, 1, 0),
    (-1, -1, 1), (0, -1, 1), (1, -1, 1), (-1, 0, 1), (0, 0, 1), (1, 0, 1), (-1, 1, 1), (0, 1, 1), (1, 1, 1),
];

fn num_neighbors(grid: &Grid, coord: &(i32, i32, i32)) -> usize {
    let (x, y, z) = coord;
    DS
        .iter()
        .filter_map(|(dx, dy, dz)| grid.get(&(x + dx, y + dy, z + dz)))
        .filter(|&&active| active)
        .count()
}

fn next_state(grid: &Grid, coord: &(i32, i32, i32)) -> bool {
    let c = *grid.get(coord).unwrap_or(&false);
    let n = num_neighbors(grid, coord);

    // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
    // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.

    match c {
        true => (2..=3).contains(&n),
        false => n == 3,
    }
}

fn advance(grid: &Grid) -> Grid {
    let mut next: Grid = HashMap::new();
    for ((x, y, z), _val) in grid.iter() {
        for (dx, dy, dz) in DS.iter() {
            let nc = (x + dx, y + dy, z + dz);
            if next.contains_key(&nc) {
                continue;  // already processed
            }
            next.insert(nc, next_state(grid, &nc));
        }
    }

    next
}

fn num_active(grid: &Grid) -> i32 {
    grid
        .iter()
        .filter(|(_cell, &occ)| occ)
        .count() as i32
}

fn process_file(path: &str) {
    let mut grid = parse_grid(path);
    // println!("Ferry: {:?}", ferry);

    // println!("Ferry:\n{}", fmt_ferry(&ferry));

    // let mut states: HashSet<String> = HashSet::new();
    for i in 1..=6 {
        grid = advance(&grid);
    }
    // println!("{}", last_ferry);
    println!("# active: {}", num_active(&grid));
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
