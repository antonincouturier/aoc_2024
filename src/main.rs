use aoc_2024::days;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Please provide a valid day number (1-25)");
        std::process::exit(1);
    });

    match day {
        1 => days::day01::run(),
        2 => days::day02::run(),
        3 => days::day03::run(),
        _ => {
            eprintln!("Day {} is not implemented", day);
            std::process::exit(1);
        }
    }
}
