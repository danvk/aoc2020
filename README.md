# Notes on Advent of Code 2020

## Day 1

I still find Rust's module system incomprehensible.

The issue this time turned out to be that I needed to import my util library as:

    use aoc2020::util;

rather than any of these:

    use crate::util;
    use super::util;
    use super::super::util;

[1]: https://stackoverflow.com/questions/60993657/cross-module-function-call-in-rust
