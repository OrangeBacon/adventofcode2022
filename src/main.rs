use std::process::ExitCode;

use clap::Parser;
use libaoc::{PathPattern, Timer};

mod day01;
mod day02;

const DAYS: &[fn(&mut Timer, input: &str)] = &[day01::solve, day02::solve];

/// Run my Advent of Code 2022 solutions
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The days to run
    #[arg(default_values_t = [DAYS.len()])]
    days: Vec<usize>,

    /// The path pattern to use to read the file containing the input data
    ///
    /// The string provided is formatted to produce the actual file path as follows:
    /// - `{}` is the day number.
    /// - `{0}` is the day number, 0 padded to be two digits.
    /// - `{{` is a literal `{`, i.e. an escape character.  If the characters after
    ///   the `{` are not `}` or `0}` then the `{` will be ignored so will not
    ///   need escaping.
    ///
    /// Formatting `a{} b{0} c{{0}}` for day 1 will produce `a1 b01 c{0}}` as an example.
    ///
    /// Limitation: Only paths that are valid utf-8 will be accepted.
    #[arg(long, short, default_value_t = String::from("./data/day{0}.txt"))]
    input: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if args.days.iter().any(|x| *x > DAYS.len()) {
        println!(
            "Specified day out of bounds: got {}, maximum {}",
            args.days.iter().max().unwrap_or(&1),
            DAYS.len(),
        );
        return ExitCode::FAILURE;
    }

    if args.days.iter().any(|x| *x == 0) {
        println!("Days are numbered 1 to n (1 indexed), there is no day 0.");
        return ExitCode::FAILURE;
    }

    let path = PathPattern::new(&args.input);

    for day in args.days {
        println!("{}", path.replace(day));

        let mut timer = Timer::now();
        DAYS[day - 1](&mut timer, "abc");

        println!("{}", timer);
    }

    return ExitCode::SUCCESS;
}
