# KENKEN Solver

I started doing those Sudoku like puzzles in the NYT Magazine while working on the crossword. I
figured why not write a solver.

My first pass was to do a mostly naive depth-first search with backtracking. It solved most 7x7
puzzles in around 10ms, so I decided not to go for the fancier exact cover approach. I also didn't
really use any heuristics other than fixing the squares with equality constraints.

My second pass was to add some heuristics to cutoff the search sooner once I could know that a
constraint was for sure going to be violated. This brought 7x7 puzzles down to about 1.5ms, and then
I took another 0.5ms off through some simple optimizations. 5x5 puzzles are solved in around 20us.

## Input

Example input:

```
7
AABBBCC
DEEFFGG
DHJLMMG
IHJNNOO
IKJJPQQ
RRRSPTU
VVWSPTU
A - 3
B * 84
C - 5
D / 3
E - 1
F / 2
G + 16
H - 1
I - 2
J + 18
K = 7
L = 2
M - 1
N + 6
O / 2
P + 10
Q - 3
R * 126
S + 8
T / 2
U + 8
V + 10
W = 1
```

The first line, `N`, is the size, either 5 or 7. The code can handle any integer up to (and including) 9,
but the magazine only has the two sizes. Followed by `N` lines each with `N` characters which
represents the puzzle grid. A unique `char` is required to represent each grouping, any ascii will
work just fine, I use capital letters. Then follows the constraints, this will be one for each
character used in the grid. There are 5 different constraint types, each with the format:

```
X o nnnn
```

where `X` is the character used in the grid above, `o` is the operation, one of `+`, `-`, `*`, `\`,
or `=`, and `nnnn` is the value from the puzzle. The `=` operation is for the squares in the puzzle
where it just tells you what the number is.

There are a few example files included in the repo, these are used with the benchmark suite.

## Running

This is built using Rust, so for speed you should probably use the release build:

```
$ cargo build --release
$ ./target/release/kenken [file]
```

where `[file]` is the path to the file containing the input data. If this is missing it is assumed
to be `puzzle.dat` in the current working directory.

If you want to see some extra output, you can use the `RUST_LOG=kenken=xxx` environment variable to
turn on logging, where `xxx` is one of `trace`, `debug`, `info`, `warn`. Each higher level gives
less information. If you want to see how many steps it took just turn on the `info` level. The lower
levels print out intermediate grids.

For example, with the above example input in `puzzle.dat` I get this on my machine:

```
[weiss:kenken (master)]$ RUST_LOG=kenken=info time ./target/release/kenken puzzle.dat
 INFO  kenken > loading puzzle.dat
 INFO  kenken::solver > Done @ 7229
2 5 3 4 7 6 1
1 4 5 3 6 7 2
3 1 6 2 4 5 7
7 2 4 5 1 3 6
5 7 2 6 3 1 4
6 3 7 1 2 4 5
4 6 1 7 5 2 3
        0.00 real         0.00 user         0.00 sys
```

So it took `7229` steps to get a solution in a small amount of time.


## Benchmarks

```
$ cargo bench
```

There are also benchmarks using criterion.rs which measure solving different size puzzles. I split
out the solving bit from the input processing step to get the speed for just that part, but I also
have a benchmark for the whole process. Spoiler: the input processing takes a negligble amount of
time.

The most recent runs on my machine for the four benchmarks are:

```
solve 5                 time:   [19.216 us 19.319 us 19.424 us]

solve 6                 time:   [36.651 us 36.824 us 37.000 us]

solve 7                 time:   [1.0095 ms 1.0173 ms 1.0266 ms]

solve 7 full            time:   [1.0213 ms 1.0263 ms 1.0317 ms]
```

They should be self-explanatory, the time is a 95% confidence interval about the mean.
