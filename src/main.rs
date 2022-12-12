use std::process::ExitCode;

use clap::Parser;
use libaoc::{DisplayRange, PathPattern, RangeParser, Timer};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

const DAYS: &[fn(&mut Timer, input: &str)] = &[
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
    day09::solve,
    day10::solve,
    day11::solve,
    day12::solve,
];

/// Run my Advent of Code 2022 solutions
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The days to run
    ///
    /// To reduce the number of parameters, ranges can be input as well as numbers.
    /// Rust's range syntax is accepted, i.e. `a..b` (exclusive of end),
    /// `a..=b` (inclusive of end), `a..` (all numbers >= a), `..b` (numbers < b),
    /// `..=b` (numbers <= b), `..` (all numbers).
    ///
    /// The smallest value acceptable is 1 and the largest is the number of the
    /// most recently solved day.
    #[arg(
        value_parser = RangeParser::new(1, DAYS.len()),
        default_values_t = [DisplayRange::one(DAYS.len())]
    )]
    days: Vec<DisplayRange>,

    /// The path pattern to use to read the file containing the input data
    ///
    /// The string provided is formatted to produce the actual file path as follows:
    /// - `{}` is the day number.
    /// - `{0}` is the day number, 0 padded to be two digits.
    /// - `{{` is a literal `{`, i.e. an escape character.  If the characters after
    ///   the `{` are not `}` or `0}` then the `{` will be ignored (removed from
    ///   the input string)
    ///
    /// Formatting `a{} b{0} c{{0}}` for day 1 will produce `a1 b01 c{0}}` as an example.
    ///
    /// Limitation: Only paths that are valid utf-8 will be accepted.
    #[arg(long, short, default_value_t = String::from("./data/day{0}.txt"))]
    input: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let path = PathPattern::new(&args.input);

    for day in args.days.into_iter().flatten() {
        let file_name = path.replace(day).into_owned();
        let file = std::fs::read_to_string(&file_name);
        let Ok(data) = file else {
            eprintln!("Unable to open data file `{file_name}`");
            return ExitCode::FAILURE;
        };

        let mut timer = Timer::now();
        DAYS[day - 1](&mut timer, &data);

        println!("{}", timer);
    }

    return ExitCode::SUCCESS;
}
