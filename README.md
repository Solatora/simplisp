# simplisp

A minimal Lisp interpreter written in Rust.

## Building

```bash
cargo build --release
```

## Running

**Interactive REPL:**
```bash
cargo run
```

**Execute a file:**
```bash
cargo run example.lisp
```

## Language Features

**Arithmetic:** `+`, `-`, `*`, `/`, `>`, `<`, `=`
**Logic:** `and`, `or`, `not`
**List Operations:** `map`, `fold`, `apply`, `split`
**File I/O:** `file-read`
**Type Conversion:** `as-number`
**Special Forms:** `define`, `if`, `function`, `quote`

## Advent of Code Example

The included `example.lisp` solves [Advent of Code 2022 Day 1](https://adventofcode.com/2022/day/1).

### Try It With Your Own Data

1. Go to [adventofcode.com/2022/day/1](https://adventofcode.com/2022/day/1) and copy your puzzle input
2. Replace the contents of `example_input.txt` with your data
3. Run:

```bash
cargo run example.lisp
```

The program outputs the answer to stdout.

> **Linux/Mac users:** Change `"\r\n\r\n"` to `"\n\n"` in `example.lisp` line 3 if your Advent of Code input uses Unix line endings.
