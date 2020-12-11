#[macro_use]
extern crate lazy_static;
use aoc2020::util;
use regex::Regex;
use std::{collections::HashSet, env, fmt};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Cell {
    Empty,
    Occupied,
    Floor,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => "L",
                Cell::Occupied => "#",
                Cell::Floor => ".",
            }
        )
    }
}

fn parse_char(c: char) -> Cell {
    match c {
        '.' => Cell::Floor,
        '#' => Cell::Occupied,
        'L' => Cell::Empty,
        _ => panic!("Invalid cell: {}", c),
    }
}

// TODO: should I make this a struct and implement Display on it?
type Ferry = Vec<Vec<Cell>>;

fn fmt_ferry(ferry: &Ferry) -> String {
    // TODO: I assume there's a more idiomatic way to do this without format! or collect
    ferry
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn parse_ferry(path: &str) -> Ferry {
    util::read_lines(path)
        .unwrap()
        .map(|line| line.unwrap())
        .map(|line| line.chars().map(parse_char).collect())
        .collect()
}

const DS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn num_neighbors(ferry: &Ferry, x: usize, y: usize) -> usize {
    let mut num = 0;
    let yi = y as i32;
    let xi = x as i32;
    for (dx, dy) in DS.iter() {
        if let Some(row) = ferry.get((yi + *dy) as usize) {
            if let Some(c) = row.get((xi + *dx) as usize) {
                if *c == Cell::Occupied {
                    num += 1;
                }
            }
        }
    }
    num
    /*
    DS
        .iter()
        .filter_map(|(dx, dy)| {
            ferry
                .get((y as i32 + dy) as usize)
                .map_or(None, |row| row.get(((x as i32) + dx) as usize))
        })
        .filter(|&&c| c == Cell::Occupied)
        .count()
        */
}

fn next_state(ferry: &Ferry, x: usize, y: usize) -> Cell {
    let c = ferry[y][x];
    let n = num_neighbors(ferry, x, y);

    match c {
        Cell::Occupied => {
            if n >= 4 {
                Cell::Empty
            } else {
                c
            }
        }
        Cell::Empty => {
            if n == 0 {
                Cell::Occupied
            } else {
                c
            }
        }
        Cell::Floor => c,
    }
}

fn advance(ferry: &Ferry) -> Ferry {
    let n = ferry.len();
    (0..n)
        .map(|y| (0..n).map(|x| next_state(ferry, x, y)).collect())
        .collect()
}

fn num_occ(ferry: &Ferry) -> i32 {
    ferry
        .iter()
        .map(|row| {
            row.iter()
                .map(|&cell| if cell == Cell::Occupied { 1 } else { 0 })
                .sum::<i32>()
        })
        .sum()
}

fn process_file(path: &str) {
    let mut ferry = parse_ferry(path);
    // println!("Ferry: {:?}", ferry);

    // println!("Ferry:\n{}", fmt_ferry(&ferry));

    // let mut states: HashSet<String> = HashSet::new();
    let mut last_ferry = String::from("");
    let mut n = 0;
    loop {
        n += 1;
        ferry = advance(&ferry);
        let s = fmt_ferry(&ferry);
        // println!("\n{} occupied: {}\n{}", n, num_occ(&ferry), s);
        if s == last_ferry {
            break;
        }
        last_ferry = s;
    }
    println!("{}, occupied: {}", n, num_occ(&ferry));
}

// 2129 = too low

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
    fn advance_33() {
        // #.#
        // ###
        let f: Ferry = vec![
            vec![Cell::Occupied, Cell::Floor, Cell::Occupied, Cell::Occupied],
            vec![Cell::Occupied, Cell::Occupied, Cell::Occupied, Cell::Occupied],
        ];
        // #.L
        // #LL
        assert_eq!(next_state(&f, 0, 0), Cell::Occupied);
        assert_eq!(next_state(&f, 1, 0), Cell::Floor);
        assert_eq!(next_state(&f, 2, 0), Cell::Empty);
    }

    #[test]
    fn test_num_neighbors() {
        // #.##
        // ####
        let f: Ferry = vec![
            vec![Cell::Occupied, Cell::Floor, Cell::Occupied, Cell::Occupied],
            vec![Cell::Occupied, Cell::Occupied, Cell::Occupied, Cell::Occupied],
        ];
        // #.L
        // #LL
        assert_eq!(num_neighbors(&f, 0, 0), 2);
        assert_eq!(num_neighbors(&f, 1, 0), 5);
        assert_eq!(num_neighbors(&f, 2, 0), 4);
    }
}
