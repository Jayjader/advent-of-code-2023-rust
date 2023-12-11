use std::fmt;
use std::fs;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Default, ValueEnum)]
enum PartOption {
    First,
    Second,
    #[default]
    Both,
}
impl fmt::Display for PartOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PartOption::First => "first",
                PartOption::Second => "second",
                PartOption::Both => "both",
            }
        )
    }
}
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long, default_value_t = PartOption::Both)]
    part: PartOption,
}

const FIRST_PART: [fn(&str) -> String; 11] = [
    day1::part1,
    day2::part1,
    day3::part1,
    day4::part1,
    day5::part1,
    day6::part1,
    day7::part1,
    day8::part1,
    day9::part1,
    day10::part1,
    day11::part1,
];
const SECOND_PART: [fn(&str) -> String; 11] = [
    day1::part2,
    day2::part2,
    day3::part2,
    day4::part2,
    day5::part2,
    day6::part2,
    day7::part2,
    day8::part2,
    day9::part2,
    day10::part1, /* to be swapped when part 2 is solved */
    day11::part2,
];
fn main() {
    let args = Args::parse();
    let input_path = format!("./input/day{}", args.day);
    let input_for_day = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Where's the input file? didn't find it at '{}'", input_path));
    if !matches!(&args.part, PartOption::Second) {
        println!(
            "day {}, part 1: {}",
            args.day,
            FIRST_PART[args.day as usize - 1](&input_for_day)
        );
    }
    if !matches!(&args.part, PartOption::First) {
        println!(
            "day {}, part 2: {}",
            args.day,
            SECOND_PART[args.day as usize - 1](&input_for_day)
        );
    }
}
