use std::env;
use std::fmt;

use aoc2020::util;
use util::read_lines;

#[derive(PartialEq)]
enum Cell {
  OPEN,
  TREE,
}

struct Forest {
  width: usize,
  height: usize,
  cells: Vec<Vec<Cell>>
}

impl fmt::Display for Forest {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Forest { width, height, cells } = self;
    let mut r = write!(f, "Width: {}, Height: {}\n", width, height);
    for row in cells {
      let cs: String = row.into_iter().map(|cell| if *cell == Cell::OPEN { '.' } else { '#' }).collect();

      r = write!(f, "{}\n", cs);
    }
    r
  }
}

fn read_forest_line(line: &str) -> Vec<Cell> {
  let x = line.chars().map(|c| { if c == '.'
  { Cell::OPEN } else { Cell::TREE } });
  let result: Vec<Cell> = x.collect();
  result
}

fn read_forest(path: &str) -> Forest {
  let cells: Vec<Vec<Cell>> = read_lines(path).unwrap().map(|line| read_forest_line(&line.unwrap())).collect();

  Forest {
    width: cells[0].len(),
    height: cells.len(),
    cells,
  }
}

fn process_file(path: &str, dx: usize, dy: usize) {
  let f = read_forest(path);

  println!("Forest:\n{}", f);

  let mut x = dx;
  let mut y = dy;
  let mut trees = 0;
  let mut open = 0;

  while y < f.height {
    let c = &f.cells[y][x % f.width];
    if *c == Cell::TREE {
      trees += 1;
    } else {
      open += 1;
    }
    x += dx;
    y += dy;
  }

  println!("Hit {} trees, {} open squares", trees, open);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 4 {
    panic!("Expected one argument, got {}: {:?}", args.len(), args);
  }

  let path = &args[1];
  let dx = args[2].parse::<usize>().unwrap();
  let dy = args[3].parse::<usize>().unwrap();

  process_file(path, dx, dy);
}
