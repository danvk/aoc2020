use std::env;
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

fn process_file(path: &str) {
  let f = read_forest(path);
  let dx = 3;
  let dy = 1;
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
  if args.len() != 2 {
    panic!("Expected one argument, got {}: {:?}", args.len(), args);
  }

  process_file(&args[1]);
}
