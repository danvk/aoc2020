use aoc2020::util;
use std::{env, fmt};

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
                .map(|c| c.to_string())
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
    DS
        .iter()
        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter_map(|(nx, ny)| {
            ferry
                .get(ny as usize)
                .and_then(|row| row.get(nx as usize))
        })
        .filter(|&&c| c == Cell::Occupied)
        .count()
}

fn next_state(ferry: &Ferry, x: usize, y: usize) -> Cell {
    let c = ferry[y][x];
    let n = num_neighbors(ferry, x, y);

    match c {
        Cell::Occupied => {
            if n >= 4 {
                Cell::Empty
            } else {
                Cell::Occupied
            }
        }
        Cell::Empty => {
            if n == 0 {
                Cell::Occupied
            } else {
                Cell::Empty
            }
        }
        Cell::Floor => Cell::Floor,
    }
}

fn advance(ferry: &Ferry) -> Ferry {
    let n = ferry.len();
    (0..n)
        .map(|y| (0..ferry[y].len()).map(|x| next_state(ferry, x, y)).collect())
        .collect()
}

fn num_occ(ferry: &Ferry) -> i32 {
    ferry
        .iter()
        .flat_map(|row| {
            row.iter()
                .filter(|&&cell| cell == Cell::Occupied)
        })
        .count() as i32
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
        // println!("\n{} occupied: {}\n{}\n---", n, num_occ(&ferry), s);
        if s == last_ferry {
            break;
        }
        last_ferry = s;
    }
    println!("{}", last_ferry);
    println!("{}, occupied: {}", n, num_occ(&ferry));
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
