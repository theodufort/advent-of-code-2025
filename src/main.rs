use advent_of_code_2025::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: cargo run -- <day> [part] [file]");
        println!("Example: cargo run -- 1");
        println!("Example: cargo run -- 1 1");
        println!("Example: cargo run -- 1 1 test.txt");
        println!("Example: cargo run -- 1 test.txt");
        return;
    }

    let day: u8 = match args[1].parse() {
        Ok(d) if d >= 1 && d <= 25 => d,
        _ => {
            eprintln!("Error: Day must be between 1 and 25");
            return;
        }
    };

    // Check if last argument is a file (contains .txt or starts with inputs/)
    let mut part: Option<u8> = None;
    let mut input_file: Option<String> = None;
    
    if args.len() >= 3 {
        // Check if arg[2] is a part number or a file
        if let Ok(p) = args[2].parse::<u8>() {
            if p == 1 || p == 2 {
                part = Some(p);
                // Check if there's a file argument after the part
                if args.len() >= 4 {
                    input_file = Some(args[3].clone());
                }
            } else {
                // Not a valid part, might be a file
                input_file = Some(args[2].clone());
            }
        } else {
            // Not a number, must be a file
            input_file = Some(args[2].clone());
        }
    }

    // Determine input file path
    let input_file_path = if let Some(file) = input_file {
        // If file doesn't start with "inputs/", prepend it
        if file.starts_with("inputs/") {
            file
        } else {
            format!("inputs/{}", file)
        }
    } else {
        format!("inputs/day{:02}.txt", day)
    };

    // Read input file
    let input = match std::fs::read_to_string(&input_file_path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Warning: Could not read input file: {}", input_file_path);
            eprintln!("Using empty input. Create the file to provide input.");
            String::new()
        }
    };

    println!("Running Day {}...", day);
    
    match day {
        1 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day01::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day01::part2(&input));
            }
        }
        2 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day02::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day02::part2(&input));
            }
        }
        3 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day03::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day03::part2(&input));
            }
        }
        4 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day04::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day04::part2(&input));
            }
        }
        5 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day05::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day05::part2(&input));
            }
        }
        6 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day06::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day06::part2(&input));
            }
        }
        7 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day07::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day07::part2(&input));
            }
        }
        8 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day08::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day08::part2(&input));
            }
        }
        9 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day09::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day09::part2(&input));
            }
        }
        10 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day10::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day10::part2(&input));
            }
        }
        11 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day11::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day11::part2(&input));
            }
        }
        12 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day12::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day12::part2(&input));
            }
        }
        13 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day13::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day13::part2(&input));
            }
        }
        14 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day14::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day14::part2(&input));
            }
        }
        15 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day15::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day15::part2(&input));
            }
        }
        16 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day16::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day16::part2(&input));
            }
        }
        17 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day17::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day17::part2(&input));
            }
        }
        18 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day18::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day18::part2(&input));
            }
        }
        19 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day19::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day19::part2(&input));
            }
        }
        20 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day20::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day20::part2(&input));
            }
        }
        21 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day21::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day21::part2(&input));
            }
        }
        22 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day22::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day22::part2(&input));
            }
        }
        23 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day23::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day23::part2(&input));
            }
        }
        24 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day24::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day24::part2(&input));
            }
        }
        25 => {
            if part.is_none() || part == Some(1) {
                println!("Part 1: {}", day25::part1(&input));
            }
            if part.is_none() || part == Some(2) {
                println!("Part 2: {}", day25::part2(&input));
            }
        }
        _ => {
            eprintln!("Error: Day {} not implemented", day);
        }
    }
}
