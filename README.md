# Notes on Advent of Code 2020

## Day 3 & 4

Rust is feeling… kind of annoying! My usual procedure for fixing errors is throwing `.unwrap()`, `&`, `String::from()` and `into_iter()` into the expression until it works.

It would be useful to find a Rust expert who's also doing the Advent of Code.

https://github.com/RoccoDev/aoc-2020/blob/master/src/days/day4.rs
https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/04.rs
https://github.com/SamMorrowDrums/aoc2/blob/day4/day4/src/main.rs
https://gist.github.com/samueltardieu/9d61cca5c6366f98e43f5719c3ae86b5
https://gist.github.com/whiter4bbit/220d30f3278b0077a08c4f28b8047eee

AxlLind's is particularly clean. I like the itertools approach, though my attempt
to factor out a helper function to separate the file's lines into blank line-delimited
"chunks" failed spectacularly. The return types for Iterators get way too complicated, and
you can't copy them from error messages because they reference closures.

The `lazy_static!` construct seems to break inference in vscode when you use it inside a
function, but not when you use it at module-level. Weird.

## Day 2

First time using regexes in Rust. Overall it seems... mostly reasonable? I got thrown off by a few things:

1. `.captures()[0]` is the full match, and `.captures()[1]` is the first capture.
2. The docs suggested using `lazy_static!` to initialize the RE once rather than in a loop, but this seemed to break type checking in VS Code.
3. ~The [docs][re] don't escape backslashes in regexes, but it seems you need to in your own code.~ Scratch that, they're using raw strings, which look like `r""`.

Also first time using `HashMap`, though in retrospect I only did this because I misinterpreted the question. The "Entry API" seems quite important. The borrow checker makes things like "iterate over all the keys and values in a hash map" surprisingly hard.

Indexing a string is also [pretty annoying][2]. I suppose this is annoyingingess that's hidden in other languages by their being slow, due to Unicode.

## Day 1

I still find Rust's module system incomprehensible.

The issue this time turned out to be that I needed to import my util library as:

    use aoc2020::util;

rather than any of these:

    use crate::util;
    use super::util;
    use super::super::util;


[1]: https://stackoverflow.com/questions/60993657/cross-module-function-call-in-rust
[2]: https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust
[re]: https://docs.rs/regex/1.4.2/regex/
