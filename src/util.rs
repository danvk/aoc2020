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
pub fn read_chunks(path: &str) -> io::Result<std::str::Split<'_, &str>> {
    Ok(std::fs::read_to_string(path)?.split("\n\n"))
}
*/

#[macro_export]
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

#[macro_export]
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
