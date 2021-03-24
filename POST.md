# Advent of Code Blog Post

I learned about the Advent of Code in mid-December of 2019. I did the puzzles in Python that year, and it always felt like I was racing to catch up. (Here's my [write-up][dan-aoc-2019].) I also felt like I was missing a chance to learn a new language. So come December 1, 2020, I was excited to start doing the puzzles as they came out, this time in Rust.

Forty nine puzzles later, I enjoyed the experience! Here are my high level takeaways:

- **This is a _great_ way to learn a language!** Not just the sequence of puzzles of increasing difficulty, but also that there are people proficient in the language solving the same problems. After day 3 I looked on r/adventofcode for people who had posted Rust solutions. I thought Axl Lind's looked particularly clean and I learned a lot about idiomatic Rust from reading it. After that, my pattern was to solve the problem myself, then go look at how Axel solved it. This worked great, except for [the one day he solved in C++][axl-cpp] instead! I think next year I'll use AoC as an opportunity to learn some Go.
- **Rust is an interesting language with lots of great ideas.** I'd choose it over C++ for a project in its domain any day. But there are many things about it that are just annoying (see below). For work that doesn't require me to work at such a low level, I'd rather use Python or TypeScript. And for writing command line programs, I suspect I'll prefer Go.
- **This year's Advent of Code was too easy.** I kept waiting for it to get hard and it never did. I missed how last year's puzzles built on one another (the IntCode computer appeared in many of them). This year's were all independent. The [global stats][stats2020] would seem to bear this out: over 12,000 people collected all 50 stars in 2020, whereas [only ~3,000][stats2019] did in 2019. There were ~50% more people who completed day 1 this year, so growth alone can't account for the 4x increase.
- **Private leaderboards are fun but dangerous** Two of my coworkers did the Advent of Code this year and we set up a private leaderboard. This was good motivation to get the puzzles solved quickly. Being on the east coast made it hard to compete on the _global_ leaderboard, particularly for later days, but I'm tempted to adjust my sleep schedule to do this next year!

So what about Rust? While I [mostly work in TypeScript these days][ets], I spent the better chunk of eight years at Google from 2006–2014 working in C++. So this level of programming is at least somewhat comfortable to me, even if I'm… rusty.

Here were a few things that struck me about the language. Day-by-day notes on the puzzles follow below.

- Rust iterators are neat! And I hear they're efficient, too. This was one of the main things I learned from reading Axel's code. I started relying on them more and more over the course of this year's AoC.
- Rust editor support isn't as good as I'm used to with TypeScript. Particularly around macros, some errors hiding other errors, `{unknown}` inferred types and the incredibly annoying "scroll through the `Vec` docs to see your error" issue. This made debugging chained method calls on iterators quite hard.
- For all the talk about the borrow checker, it's pretty rare that you actually use an explicit lifetime annotation. I only used [one][lifetime1] or [two][lifetime2] in the whole Advent of Code.
- Understanding that `String` is for strings you own was helpful. (Otherwise you can use `&str`.) I often wound up resolving borrow checker errors by throwing in a `String::from` or `.clone()`. Not the most efficient, but certainly effective.
- Rust never implicitly casts between numeric types. I understand the argument for this and like it in principle. But wow is it annoying in practice! Having to constantly convert back and forth to `usize` to index vectors got quite cumbersome. Particularly annoying is that you can't do `x[i + (-1)]` if `i` has type `usize`. Instead, you have to do `x[((i as i32) + (-1)) as usize]`. This [came up a lot][as-usize] in problems where you could move in any direction on a grid. Is there a more idiomatic way to do this without all the casting?
- Inference works a bit differently than in TS. It's common for a type to be set based on later usage (e.g. a `Vec`). But return types for functions cannot be inferred. It's neat that you can write somthing like `iter.collect::<Vec<_>>()` to be explicit about the `Vec` but let the generic type be inferred. I missed being able to see inferred types midway through chains like you can in TypeScript (hover over a generic function to see which type parameters were inferred for it). The inability to infer return types of functions was particularly frustrating when the type I wanted to return couldn't be written down, e.g. because it involved a closure.
- There are some forms of abstraction that the borrow checker makes [either impossible or hard][axl-discussion]. For example, writing a function that consumes an iterator, filters & maps it, and returns another iterator. Does writing a function that processes an iterator depend on having garbage collection?
- It's worth learning all the methods on [`Option`s][option] and [`Result`s][result] since they come up so often. I wound up with `unwrap()` everywhere in my code. I wish you could use `?` syntax with `Option` in addition to `Result`. Learning `and_then` was [useful on Day 11][and-then].
- Coming from TypeScript, I found it a bit surprising that individual variants of an Enum aren't types. You can't write `let mask: Op::Mask = Op::Mask { ... };`.
- While it stopped being an issue after day 1, I still find the Rust project structure (with `lib` and `bin`) pretty baffling. I wanted my project to be mostly binaries with a few shared modules between them. After much flailing, I wound up with something that worked. But I'm still mystified as to when I have to write `use aoc2020::util;` (the package name) vs. being able to write `use super::util;` (as in [rusty boggle][])
- For non-primitive types, it makes sense that functions like `map` and `filter` borrow the values over which they iterate. But since I was often working with `Vec<i32>`, the referencing and dereferencing felt like overkill. I also had a hard time figuring out when you could destructure borrows (e.g. write `for &x in ...`) and when you couldn't. The meaning of `&&i32` isn't entirely clear to me. Should I think of references as being like pointers? Is this the same as C++, where a reference is just a non-nullable pointer?
- I like how the error messages in Rust refer to GitHub issues and RFCs. For example, writing `(dx, dy) = (-dy, dx)` refers you to <https://github.com/rust-lang/rfcs/issues/372>. This makes the language development process feel very accessible.
- There are a few missing constructs that feel like they should be built in, e.g. a `HashMap` or `HashSet` literal. I'm sure Rust will add these eventually. I eventually learned that you can [write a macro][map-macro] to add this feature yourself. Pretty cool! I've never worked in a language that had macros like this before. They seem quite powerful.
- Static values are quite limited in Rust. I'm not sure you can create a static `Vec` without `lazy_static`. I found it surprising that you have to write out array lengths for statics, for example.

### Daily notes

#### Day 1: Report Repair

[problem](https://adventofcode.com/2020/day/1) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day1.rs)

I still find Rust's module system incomprehensible.

The issue this time turned out to be that I needed to import my util library as:

    use aoc2020::util;

rather than any of these:

    use crate::util;
    use super::util;
    use super::super::util;

#### Day 2: Password Philosophy

[problem](https://adventofcode.com/2020/day/2) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day2.rs)

First time using regexes in Rust. Overall it seems... mostly reasonable? I got thrown off by a few things:

1. `.captures()[0]` is the full match, and `.captures()[1]` is the first capture.
2. The docs suggested using `lazy_static!` to initialize the RE once rather than in a loop, but this seemed to break type checking in VS Code.

Also first time using `HashMap`, though in retrospect I only did this because I misinterpreted the question. The "Entry API" seems quite important. The borrow checker makes things like "iterate over all the keys and values in a hash map" surprisingly hard.

Indexing a string is also [pretty annoying][2]. I suppose this is annoyingingess that's hidden in other languages by their being slow, due to Unicode.

#### Day 4: Passport Processing

[problem](https://adventofcode.com/2020/day/4) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day4.rs)

Rust is feeling… kind of annoying! My usual procedure for fixing errors is throwing `.unwrap()`, `&`, `String::from()` and `into_iter()` into the expression until it works.

It would be useful to find a Rust expert who's also doing the Advent of Code. Here are a few I found on r/adventofcode: [RoccoDev](https://github.com/RoccoDev/aoc-2020/blob/master/src/days/day4.rs), [AxlLind](https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/04.rs), [SamMorrowDrums](https://github.com/SamMorrowDrums/aoc2/blob/day4/day4/src/main.rs), [samueltardieu](https://gist.github.com/samueltardieu/9d61cca5c6366f98e43f5719c3ae86b5), [whiter4bbit](https://gist.github.com/whiter4bbit/220d30f3278b0077a08c4f28b8047eee).

AxlLind's is particularly clean. I like the itertools approach, though my attempt to factor out a helper function to separate the file's lines into blank line-delimited "chunks" failed spectacularly. The return types for Iterators get way too complicated, and you can't copy them from error messages because they reference closures.

The `lazy_static!` construct seems to break inference in vscode when you use it inside a function, but not when you use it at module-level. Weird.

#### Day 5: Binary Boarding

[problem](https://adventofcode.com/2020/day/5)

I was feeling some social pressure to get out the door this morning, so I just did this one with find/replace, `sort`, `bc` and a spreadsheet.

    (echo 'ibase=2'; cat inputs/day5.txt | perl -pe 's/B/1/g; s/F/0/g; s/R/1/g; s/L/0/g;' | sort) | bc | pbcopy

#### Day 6: Custom Customs

[problem](https://adventofcode.com/2020/day/6) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day6.rs)

I was able to reuse the chunking code from day 4 to make short work of this.

I'm still puzzled at why it's so difficult to factor out functions that work with Iterators in Rust. It seems very easy to get into situations where the return type involves closures that can't be written in the type system, or to get into trouble with the borrow checker.

I looked for a functional way to do "count by" on the characters in a string but wasn't able to find anything.

#### Day 8: Handheld Halting

[problem](https://adventofcode.com/2020/day/8) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day8.rs)

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

moved more logic into the parsing but simplified everything downstream. It does feel odd to me that I can't name the parameters in each case of the enum (`arg: i32` instead of just `i32`). _Update: Axel tells me that I can!_

I learned about `filter_map`, which combines `map` with unwrapping `Option`s. This seems great, but I haven't been able to use it yet because I usually want to unwrap `Result`s.

#### Day 9: Encoding Error

[problem](https://adventofcode.com/2020/day/9) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day9.rs)

Got tripped up a bit by [`.combinations()`][combinations] not working as I'd expected:

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

#### Day 10: Adapter Array

[problem](https://adventofcode.com/2020/day/10) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day10.rs)

Part two was the first puzzle where brute force was too slow. My first instinct was to do the search from both sides, to `sqrt` the runtime. But getting the join condition just right is tricky. Then I realized there are some joltages that you _have_ to go through. So those are the natural breakpoints. From there it was a fight with off-by-one errors.

One thing I find confusing about Rust iterators... why is the type of `x` here `&i32` and not just `i32`?

    let jolts: &[i32];
    jolts.iter().map(|x| x);

#### Day 11: Seating System

[problem](https://adventofcode.com/2020/day/11) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day11.rs)

My code for part 1 worked just great on the sample code, but produced an incorrect answer on my input. A bug! These tend to be quite frustrating with AoC since you only know the correct output for what they give you. I tried a few different things but to no avail. My unit tests were passing but I was producing the wrong answer.

I eventually got frustrated and [reimplemented the whole thing in Python][py11]. It only took maybe 10 minutes. It gave me the correct answer and, more importantly, correct intermediate states to compare against my broken Rust program. As it turned out, while the sample input was square (10x10), my personal input was slightly off-square (92x91). Sneaky! And I'd made an assumption about a square grid in one part of my code. Once I fixed the bug, part two was pretty easy.

[Jack][jack] said he had a unit test for a 3x2 grid. In retrospect that would have been a good way to do it, too. Unit tests for slightly higher level functions than "count your neighbors".

I learned a few things from [Axel's code for Day 11][axl-11]:

- There's an `unreachable!()` macro you can use instead of `panic!("reason")`. Not exactly sure why you'd prefer this.
- He used `get` and `and_then` to chain map lookups. I was looking for `and_then`! I tried using `map_or` but ran into some bugs that I couldn't figure out. I wound up rewriting this in a more imperative style.
- He also wrote out an eight-element tuple of directions (`DS`). This was a source of confusing bugs for me, as I had a duplicate / missing entry in mine.
- Axel tends to separate his `as i64` from his `as usize`, the latter only appearing at the place where you index into an array.
- Using `|&&x|` in a lambda is OK.
- Apparently you can do this (`i` and `j` being parameters): `let (mut i, mut j) = (i as i64, j as i64);`

It's not always clear to me when you need to write `.iter()` before `.map()` and when you can just write `.map()`. Or when you need to write `.collect()`. Why do I have to `.collect()` an iterator of `String`s before calling `join()`?

You can use `use EnumType::*` to drop the need to qualify its contituents.

#### Day 12: Rain Risk

[problem](https://adventofcode.com/2020/day/12) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day12.rs)

Hopefully continuing the pattern of easy puzzles on the weekend. I woke up early and wanted to do AoC, so this wound up being my best result so far (17726 / 14724 — tough to get a top result on the east coast!).

A few things of note:

- (-90) % 360 = -90 in Rust
- I wanted to write `(dx, dy) = (-dy, dx)` to rotate 90°, but was referred to <https://github.com/rust-lang/rfcs/issues/372>
- Storing the waypoint delta, instead of its absolute position, wound up being a good choice.

#### Day 13: Shuttle Search

[problem](https://adventofcode.com/2020/day/13) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day13.rs)

Relieved that I didn't overflow `u64` today! My modular math is quite rusty (I took college Algebra in… 2003?) so while I was pretty confident there was a canonical solution to a system of equations over different moduli, I didn't remember exactly what it was. I was pretty confident that I could solve subproblems by figuring out what the number was mod p1*p2 if p1 and p2 were relatively prime.

I _was_ able to come up with an answer this way, but unfortunately it wasn't the smallest answer on the sample problem. I got:

    // primes: [(59, 4), (31, 6), (19, 7), (13, 1), (7, 0)]
    n = 2093560 (mod 3162341)

but the solution was 1068781. I noticed that this was close to the difference of those two numbers, so I tried it… and `3162341 - 2093560 = 1068781`! So I tried this on the big problem and it worked.

Looking back at this, I had the congruences messed up. If you want bus 19 to show up one timestamp after bus 7, then you need `n + 1 = 0 (mod 19)`, not `n = 1 (mod 19)`. That explains why I had the answer exactly backwards! My solution did work, I was just solving the wrong problem. In retrospect, writing more tests on small inputs would have helped me find this.

There are very efficient ways to calculate the multiplicative inverse of a number mod a prime, but my brute force solution worked fine in practice.

Apparently this problem is just the [Chinese Remainder Theorem][crt]. One other wrinkle that tripped me up: because of the way the problem is constructed, sometimes the residue was larger than the prime. So you have to do some addition / modulus in the problem setup, too.

#### Day 14: Docking Data

[problem](https://adventofcode.com/2020/day/14) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day14.rs)

This one wasn't very challenging, just had to work it out and get all the bit shifting and masking right. I used a loop from `0..2.pow(n)` to iterate over all possible combinations for the "floating" bits in part 2. I was wondering if Axl would come up with some Rust standard library function for this, but apparently not.

One thing I was surprised by in Rust: there's a big distinction between an enum, which is a type, and a _variant_ of the enum, which is not. So while I can declare:

    let mask: Op = ...;

I cannot declare:

    let mask: Op::Mask = Op::Mask { ... };

Not really clear to me why you wouldn't want to allow this. It works great in TypeScript.

Another thing I learned: to ignore a field while destructuring / matching, you assign it to `_`:

    Op::Mask { ones, zeros: _, xs }

#### Day 15: Rambunctious Recitation

[problem](https://adventofcode.com/2020/day/15) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day15.rs)

Slightly annoying to avoid off-by-ones, but after that this was quite fast. I was happy that you can get away only storing the last round for each number, as opposed to the previous two or N. I wonder if the 30,000,000 rounds in step 2 is a problem if you implement this in a slow way, or in Python? It took ~2 secs with Rust:

    $ cargo run --release --bin day15 0,20,7,16,1,18,15 30000000
    Compiling aoc2020 v0.1.0 (/Users/danvk/github/aoc2020)
        Finished release [optimized] target(s) in 0.52s
        Running `target/release/day15 0,20,7,16,1,18,15 30000000`
    nums: [0, 20, 7, 16, 1, 18, 15]
    last spoken: 129262 after 30000000 rounds (2317 ms)

I was curious so I ported my solution to Python. It's ~6x slower:

    $ time python3 py/day15.py 0,20,7,16,1,18,15 30000000
    After 30000000, last_spoken=129262
    python3 py/day15.py 0,20,7,16,1,18,15 30000000  13.51s user 0.18s system 99% cpu 13.721 total

You read everywhere that Rust's default hasher for HashMap is "known to be slow for small keys like ints." I swapped in `rustc_hash::FxHashMap` but didn't get too much of a speedup, only down to ~1.7 secs.

Switching from a `HashMap` to a long `Vec` had a bigger impact on performance, getting me down to 688ms.

#### Day 17: Conway Cubes

[problem](https://adventofcode.com/2020/day/17) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day17.rs)

The trick I learned [last year][2019] of representing grids using maps from coordinate tuples to values made this one a lot easier! Almost no change from part 1 to part 2. The only trick was making sure you considered the next state for all _neighbors_, not all cells.

Was nice to learn that you can use `lazy_static!` to fill a vector. This makes enumerating all the combinations of -1, 0, +1 for directions a lot less error-prone than writing them by hand:

```rust
lazy_static! {
    static ref DS: Vec<(i32, i32, i32, i32)> = {
        let mut v: Vec<(i32, i32, i32, i32)> = vec![];
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx != 0 || dy != 0 || dz != 0 {
                        v.push((dx, dy, dz, dw));
                    }
                }
            }
        }
        assert_eq!(26, v.len());
        v
    };
}
```

I don't know if there's any downside to representing the directions this way vs. as `[(i32, i32, i32); 26]`. You can make either work, the latter just requires a little more care.

#### Day 18: Operation Order

[problem](https://adventofcode.com/2020/day/18) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day18.rs)

One of my project ideas for learning some Rust was implementing an answer to [this question][cbc], a command-line calculator that ignores commas and dollar signs. I'd poked around at [pest][] a month ago, but it was a bit beyond my Rust abilities at the time. While a full parser generator is a bit overkill for this problem (certainly for part 1!) this seemed like a good opportunity to try again. I was pleasantly surprised that it was much easier to get it working this time. I must have learned some Rust in the past month!

There's a lot of macro magic involved in Pest. It works, but one downside is that you completely lose the types in your editor:

![unknown type](screenshots/pest-unknown.png)

#### Day 19: Monster Messages

[problem](https://adventofcode.com/2020/day/19) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day19.rs)

I half expected my part one solution to work for part 2, but I think it would require not being greedy.

I thought about implementing some sort of lookahead to ensure that each instantiation of a rule consumed at least one character. But then I looked at the recursive rules and realized that there was a much simpler pattern. You need some 42s, and then a smaller number of 31s.

At this point I felt pretty confused! I felt like I'd figured it out, but couldn't see how their matching worked out for the sample. I kept getting a smaller number of matches.

Eventually I tracked it down to a bug in my code that found all the strings that matched a rule. I'd changed a `pieces[0]` to `pieces.pop()`, which had the effect of permuting some of the matches. It didn't throw me off enough to prevent the key insight to solve the problem (rules 31 and 42 match disjoint strings of the same length) but did give me confusingly-wrong answers.

This one definitely took me the longest! As usual, more thorough unit tests might have caught the bug. I just thought I was misinterpreting the rules.

On the plus side, I used my first lifetime annotation today!

    fn match_str<'a>(&self, txt: &'a str, rules: &HashMap<i32, Rule>) -> Option<&'a str> {

This consumes a `str` and returns a slice of it, so they have the same lifetime.

I also learned about `r#""#`-style strings, which allow `"` characters in the raw string.

Two issues I ran into today:

- The borrow checker error that led to the signature above didn't show up in VS Code, even after restarting the Rust server. It only showed up as an error when I ran the program.
- I'm getting quite annoyed at errors showing up after extremely long documentation strings. You have to scroll all the way down through several pages of text to see the error. And if you scroll even a pixel too far, the whole dialog goes away.

There's no nice syntax for map or set literals (see [RFCs #542][542]) but it's pretty easy to write a function to make this more pleasant. You can also make a macro to do this, though the issue of `&str` vs. `String` seems worse there. Macros seem pretty cool! I'd like to see what else people use them for.

#### Day 20: Jurassic Jigsaw

[problem](https://adventofcode.com/2020/day/20) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day20.rs)

Today was pretty rough! I think my whole approach was more or less fine, it was just a slog to implement all the rotations and flips. I wound up writing out all the transformations (flips + rotates) on paper to make sure I got them right. After yesterday's experience, I was being careful.

The one clever thing I did was checking how many possible neighbors there were for each cell. For both the sample and my input, there were exactly four tiles with only two possible neighbors. Since part one only required the product of the tile IDs in the corners, that was enough. I didn't have to solve the puzzle to get my star. So I beat Jack and Jeremy to the first star!

For the second part, I figured I could pick an arbitrary corner tile as the top left and figure out how to attach the neighbors to its right and bottom. That would set the orientation for the whole puzzle. Then I could push out my solution diagonally towards the bottom right. This worked great, it just took me a while to implement and get it right.

In retrospect, I should have just stored the cells in a tile and made `left()`, `right()` etc. be methods. This didn't wind up being a performance-sensitive problem.

Rust notes:
- Wrote macros for `map!` and `set!` literals in `util.rs`. For some reason these are `aoc2020::map` and not `aoc2020::util::map`.
- It's confusing to me when you can do `for &x in ...` and when you can't.
- `.collect`ing an iterator of pairs into a hash map is a pretty neat pattern.
- I think "Missing lifetime annotation" errors don't show up in VS Code and prevent any other errors from showing up, either.

#### Day 23: Crab Cups

[problem](https://adventofcode.com/2020/day/23) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day23.rs)

I implemented part 1 with `Vec`, thinking in the back of my mind that a circular linked list would make more sense. But with only 100 rounds, why?

For part 2, I was vaguely hopeful that 1M cards and 10M rounds would still be small enough for my part 1 solution to work. I think it _would_ have, but only after ~10 hours. Interestingly, Jeremy reported that [his Python implementation][jeremy23] using a list would also have taken ~10 hours. I guess most of that time is big calls to `memcpy`, whether it's being done by Rust or Python. So it makes sense that they take about the same.

I started reading about [Rust's `std::LinkedList`][ll] but it seemed pretty terrible. The "cursors" feature is experimental and many of the operations that would make you want to use a linked list, like `remove`, have APIs that force them to be O(N) instead of O(1). There were also some [blog posts][lists] and Stack Overflow questions that suggested that writing a linked list implementation is quite hard in Rust due to unclear ownership.

That got me thinking about how I could make my own hacky linked list. I started with a `Vec` of the numbers and set up a parallel array to store the index of the next value in the list. Then I realized I didn't need the first array at all! The value at position N is the number after N in the list. Once I switched to this representation, all the game operations became completely trivial and I got the solution in 680ms.

Apparently Axl switched to C for this one! I think knowing the [100 prisoners problem][100] made the permutation array representation more intuitive to me.

#### Day 24: Lobby Layout

[problem](https://adventofcode.com/2020/day/24) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day24.rs)

The only tricky part here was representing the hex grid and avoiding off-by-one errors. I keep getting tripped up by `-1 % 2 == -1`. I learned about Rust's `chain` today, which is a handy way to chain two iterators together. I used this to iterate over a cell _and_ its neighbors.

#### Day 25: Combo Breaker

[problem](https://adventofcode.com/2020/day/25) / [solution](https://github.com/danvk/aoc2020/blob/master/src/bin/day25.rs)

Just like last year, day 25 was an easy one-parter. I had fun doing AoC this year and certainly learned a lot of Rust! But it felt a _lot_ easier than last year's. I kept expecting it to get hard and, for the most part, it didn't. These were the only days that required much thought & care:

- Day 13: bus arrival times
- Day 19: a/b message parsing
- Day 20: solving a puzzle w/ rotation & flips
- Day 23: crab cups

Next year I think I'll do it in [Go][go]!

[rusty boggle]: https://github.com/danvk/rusty-boggle/blob/47f91bf1d06ff9abf743daac1a7dcebd4bdd226e/src/boggler.rs#L1
[as-usize]: https://github.com/danvk/aoc2020/blob/c6937280fc2b514d2108312584b72f4d0bd9ee9d/src/bin/day9.rs#L24
[and-then]: https://github.com/danvk/aoc2020/blob/c6937280fc2b514d2108312584b72f4d0bd9ee9d/src/bin/day11.rs#L77
[option]: https://doc.rust-lang.org/std/option/enum.Option.html
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[map-macro]: https://github.com/danvk/aoc2020/blob/c6937280fc2b514d2108312584b72f4d0bd9ee9d/src/util.rs#L18-L42
[lifetime1]: https://github.com/danvk/aoc2020/blob/c6937280fc2b514d2108312584b72f4d0bd9ee9d/src/bin/day20.rs#L224
[lifetime2]: https://github.com/danvk/aoc2020/blob/c6937280fc2b514d2108312584b72f4d0bd9ee9d/src/bin/day21.rs#L21-L25
[axl-cpp]: https://github.com/AxlLind/AdventOfCode2020/blob/master/notes.md#day-23---link-1180560
[axl-discussion]: https://github.com/danvk/aoc2020/commit/08c254675af2b9e78a84df9387d1493213e34e8e#commitcomment-44854049

[1]: https://stackoverflow.com/questions/60993657/cross-module-function-call-in-rust
[2]: https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust
[re]: https://docs.rs/regex/1.4.2/regex/
[2019]: https://medium.com/@danvdk/python-tips-tricks-for-the-advent-of-code-2019-89ec23a595dd
[cbc]: https://softwarerecs.stackexchange.com/q/75993/69199
[pest]: https://pest.rs
[pest-intro]: https://pest.rs/book/intro.html
[542]: https://github.com/rust-lang/rfcs/issues/542
[ll]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
[lists]: https://rust-unofficial.github.io/too-many-lists/
[100]: https://en.wikipedia.org/wiki/100_prisoners_problem
[stats2020]: https://adventofcode.com/2020/stats
[stats2019]: https://adventofcode.com/2019/stats
[dan-aoc-2019]: https://medium.com/@danvdk/python-tips-tricks-for-the-advent-of-code-2019-89ec23a595dd
[combinations]: https://docs.rs/itertools/0.7.8/itertools/structs/struct.Combinations.html
[py11]: https://github.com/danvk/aoc2020/blob/master/py/day11.py
[jack]: https://github.com/jamadeo/adventofcode/tree/master/day_11
[axl-11]: https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/11.rs
[crt]: https://en.wikipedia.org/wiki/Chinese_remainder_theorem
[jeremy23]: https://github.com/docmarionum1/adventofcode/blob/main/2020/23/Crab%20Cups.ipynb
[go]: https://golang.org/
[ets]: https://effectivetypescript.com/
