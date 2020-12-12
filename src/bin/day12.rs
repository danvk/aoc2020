use aoc2020::util;
use std::{env, fmt};

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    wdx: i32,
    wdy: i32,
    /// degrees, 0 = east
    dir: i32,
}

#[derive(Debug)]
enum Action {
    F(i32),
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
}

fn parse_action(s: &str) -> Action {
    let c = s.chars().nth(0).unwrap();
    let arg = *&s[1..].parse::<i32>().unwrap();
    match c {
        'N' => Action::N(arg),
        'S' => Action::S(arg),
        'E' => Action::E(arg),
        'W' => Action::W(arg),
        'L' => Action::L(arg),
        'R' => Action::R(arg),
        'F' => Action::F(arg),
        _ => panic!("Invalid cell: {}", c),
    }
}

fn dir(degrees: i32) -> (i32, i32) {
    match (degrees + 3600) % 360 {
        0 => (1, 0),
        90 => (0, 1),
        180 => (-1, 0),
        270 => (0, -1),
        _ => unreachable!()
    }
}

fn rot(d: (i32, i32), degrees: i32) -> (i32, i32) {
    let mut degrees = ((degrees % 360) + 360) % 360;
    let (mut dx, mut dy) = d;

    while degrees > 0 {
        degrees -= 90;
        let ndx = -dy;
        let ndy = dx;
        dx = ndx;
        dy = ndy;
    }
    assert_eq!(0, degrees);

    (dx, dy)
}

fn process_file(path: &str) {
    let mut ship = Ship { x: 0, y: 0, wdx: 10, wdy: 1, dir: 0 };
    for line in util::read_lines(path).unwrap().map(|line| line.unwrap()) {
        let action = parse_action(&line);

        match action {
            Action::N(arg) => ship.wdy += arg,
            Action::S(arg) => ship.wdy -= arg,
            Action::E(arg) => ship.wdx += arg,
            Action::W(arg) => ship.wdx -= arg,
            Action::L(arg) => {
                let (dx, dy) = rot((ship.wdx, ship.wdy), arg);
                ship.wdx = dx;
                ship.wdy = dy;
            },
            Action::R(arg) => {
                let (dx, dy) = rot((ship.wdx, ship.wdy), -arg);
                ship.wdx = dx;
                ship.wdy = dy;
            },
            Action::F(arg) => {
                ship.x += arg * ship.wdx;
                ship.y += arg * ship.wdy;
            }
        }
        println!("{:?} -> {:?}", &action, &ship);
    }
    println!(
        "Manhattan distance: {} + {} = {}",
        ship.x.abs(), ship.y.abs(),
        ship.x.abs() + ship.y.abs(),
    )
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
    fn test_mod() {
        let x = -270 - 360;
        assert_eq!(-270, x % 360);

        assert_eq!(90, ((x % 360) + 360) % 360);
    }

    #[test]
    fn test_rot() {
        assert_eq!((2, 1), rot((2, 1), 0));
        assert_eq!((-1, 2), rot((2, 1), 90));
        assert_eq!((-2, -1), rot((2, 1), 180));
        assert_eq!((1, -2), rot((2, 1), 270));
        assert_eq!((1, -2), rot((2, 1), -90));
        assert_eq!((-2, -1), rot((2, 1), -180));
        assert_eq!((-1, 2), rot((2, 1), -270));
        assert_eq!((2, 1), rot((2, 1), 360));
        //        (dx, dy)
        // +90 -> (-dy, dx)
        // +180 -> (-dx, -dy)
        // +270 -> (dy, -dx)
    }
}
