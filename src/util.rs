use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Helper from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/*
pub fn read_lines2(filename: &str) -> Vec<&str> {
    let x: Vec<String> = std::fs::read_to_string(filename).unwrap().split("\n").map(|x| String::from(x).collect();
    x
}
*/
