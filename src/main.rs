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
        4 => days::day04::run(),
        5 => days::day05::run(),
        6 => days::day06::run(),
        7 => days::day07::run(),
        8 => days::day08::run(),
        9 => days::day09::run(),
        10 => days::day10::run(),
        11 => days::day11::run(),
        _ => {
            eprintln!("Day {} is not implemented", day);
            std::process::exit(1);
        }
    }
}
