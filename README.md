# KENKEN Solver

I started doing those Sudoku like puzzles in the NYT Magazine while working on the crossword. I
figured why not write a solver.

My first pass was to do a mostly naive depth-first search with backtracking. It solves most 7x7
puzzles in around 10ms, so I decided not to go for the fancier exact cover approach. I also don't
really use any heuristics other than fixing the squares with equality constraints.

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

The first line, `N`, is the size, either 5 or 7. The code can handle any integer up to (and including) 7,
but the magazine only has the two sizes. Followed by `N` lines each with `N` characters which
represents the puzzle grid. A unique char is required to represent each grouping, any ascii will
work just fine, I use capital letters. Then is follows the constraints, this will be one for each
character used in the grid. There are 5 different constraint types, each with the format:

```
X o nnnn
```

where `X` is the character used in the grid above, `o` is the operation, one of `+`, `-`, `*`, `\`,
or `=`, and `nnnn` is the value from the puzzle. The `=` operation is for the squares in the puzzle
where it just tells you what the number is.

## Running

This is built using Rust, so

```
$ cargo build --release`
$ ./target/release/kenken [file]
```

where `[file]` is the path to the file containing the input data. If this is missing it is assumed
to be `puzzle.dat` in the current working directory.
