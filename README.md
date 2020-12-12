# Notes on Advent of Code 2020

## Day 12

Hopefully continuing the pattern of easy puzzles on the weekend. I woke up early and wanted to do AoC, so this wound up being my best result so far (17726 / 14724 — tough to get a top result on the east coast!).

A few things of note:

- (-90) % 360 = -90 in Rust
- I wanted to write `(dx, dy) = (-dy, dx)` to rotate 90°, but was referred to https://github.com/rust-lang/rfcs/issues/372
- Storing the waypoint delta, instead of its absolute position, wound up being a good choice.

## Day 11

My code for part 1 worked just great on the sample code, but produced an incorrect answer on my input. A bug! These tend to be quite frustrating with AoC since you only know the correct output for what they give you. I tried a few different things but to no avail. My unit tests were passing but I was producing the wrong answer.

I eventually got frustrated and reimplemented the whole thing in Python. It only took maybe 10 minutes. It gave me the correct answer and, more importantly, correct intermediate states to compare against my broken Rust program. As it turned out, while the sample input was square (10x10), my personal input was slightly off-square (92x91). Sneaky! And I'd made an assumption about a square grid in one part of my code. Once I fixed the bug, part two was pretty easy.

Jack said he had a unit test for a 3x2 grid. In retrospect that would have been a good way to do it, too. Unit tests for slightly higher level functions than "count your neighbors".

Notes from Axel's code:

- There's an `unreachable!()` macro you can use instead of `panic!("reason")`.
  Not exactly sure why you'd prefer this.
- He used `get` and `and_then` to chain map lookups. I was looking for `and_then`!
  I tried using `map_or` but ran into some bugs that I couldn't figure out.
  I wound up rewriting this in a more imperative style.
- He also wrote out an eight-element tuple of directions (`DS`).
  This was a source of confusing bugs for me, as I had a duplicate / missing entry in mine.
- Axel tends to separate his `as i64` from his `as usize`, the latter only appearing at the
  place where you index into an array.
- Using `|&&x|` in a lambda is OK.
- Apparently you can do this (`i` and `j` being parameters):

    let (mut i, mut j) = (i as i64, j as i64);

It's not always clear to me when you need to write `.iter()` before `.map()` and when you can
just write `.map()`. Or when you need to write `.collect()`. Why do I have to `.collect()` an
iterator of `String`s before calling `join()`?

You can use `use EnumType::*` to drop the need to qualify its contituents.

## Day 10

Part two was the first puzzle where brute force was too slow. My first instinct was to do the search from both sides, to sqrt the runtime. But getting the join condition just right is tricky. Then I realized there are some joltages that you _have_ to go through. So those are the natural breakpoints. From there it was a fight with off-by-one errors.

One thing I find confusing about Rust iterators... why is the type of `x` here `&i32` and not just `i32`?

    let jolts: &[i32];
    jolts.iter().map(|x| x);

## Day 9

Got tripped up a bit by `.combinations()` not working as I'd expected:

```rust
fn is_pair_sum(n: u64, nums: &[u64]) -> bool{
    // TODO: why can't I make the lambda look like: |(a, b)| a + b == n?
    nums.iter().combinations(2).any(|x| x[0] + x[1] == n)
}
```

The `x` here is `Vec<&u64>`. Looking at Axl's code, it turns out that I need to use `tuple_combinations` instead:

```rust
fn is_pair_sum(n: u64, nums: &[u64]) -> bool{
    nums.iter().tuple_combinations().any(|(a, b)| a + b == n)
}
```

Why would I ever _not_ want `tuple_combinations`? It feels a little magical to me that `tuple_combinations` is able to infer the `2` from the signature of the `any` callback. How does that work?

I had to switch from `i32` to `i64` for accumulating. (I got an overflow panic, which was much more helpful than the incorrect results you'd get in C.) I'm nervous what will happen if I need to go higher than that. I remember that Python's bigints were quite helpful for last year's IntCode computer.

The constant conversions between `i32` and `usize` for indexing are pretty annoying. You need a `usize` to index. But if you ever want to subtract one, you need to convert it to an `i32` first. Hence this grossness:

```rust
is_pair_sum(n, &nums[(i as i32 - preamble_len) as usize..i])
```

Iterating from short subsequences to long makes a huge time difference, even though both are O(N^3):

    Time: 2961ms
    Time: 7ms



## Day 8

Our first problem involving implementing a computer. Switching from a struct:

```rust
struct Instruction {
    op: String,
    arg: i32,
}
```

to an enum:

```rust
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}
```

moved more logic into the parsing but simplified everything downstream. It does feel odd to me that I can't name the parameters in each case of the enum (`arg: i32` instead of just `i32`).

I learned about `filter_map`, which combines `map` with unwrapping `Option`s. This seems great, but I haven't been able to use it yet because I usually want to unwrap `Result`s.

## Day 6

I was able to reuse the chunking code day 4 to make short work of this.

I'm still puzzled at why it's so difficult to factor out functions that work with Iterators in Rust. It seems very easy to get into situations where the return type involves closures that can't be written in the type system, or to get into trouble with the borrow checker.

I looked for a functional way to do "count by" on the characters in a string but wasn't able to find anything.

## Day 5

I was feeling some social pressure to get out the door this morning, so I just did this one with find/replace, `sort`, `bc` and a spreadsheet.

    (echo 'ibase=2'; cat inputs/day5.txt | perl -pe 's/B/1/g; s/F/0/g; s/R/1/g; s/L/0/g;' | sort) | bc | pbcopy

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
