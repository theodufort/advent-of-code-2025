# Advent of Code 2025

This repository contains solutions for Advent of Code 2025, organized by day.

## Structure

- `src/day01.rs` through `src/day25.rs` - Individual day solutions
- `src/lib.rs` - Module declarations
- `src/main.rs` - Main entry point to run solutions
- `inputs/` - Directory for input files (create `day01.txt`, `day02.txt`, etc.)

## Usage

### Running a day's solution

Run both parts:
```bash
cargo run -- 1
```

Run a specific part:
```bash
cargo run -- 1 1  # Run part 1 only
cargo run -- 1 2  # Run part 2 only
```

### Adding input files

Place your input files in the `inputs/` directory with the naming pattern:
- `inputs/day01.txt`
- `inputs/day02.txt`
- etc.

### Implementing a solution

Each day file (`src/dayXX.rs`) contains:
- `part1(input: &str) -> String` - Solution for part 1
- `part2(input: &str) -> String` - Solution for part 2
- Test module with example tests

Simply replace the `"Not implemented"` placeholder with your solution logic.

### Running tests

```bash
cargo test
```

To run tests for a specific day:
```bash
cargo test day01
```

