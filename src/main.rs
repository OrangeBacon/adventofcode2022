use std::process::ExitCode;

use clap::Parser;
use libaoc::Timer;

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

    for day in args.days {
        let mut timer = Timer::now();
        DAYS[day - 1](&mut timer, "abc");

        println!("{}", timer);
    }

    return ExitCode::SUCCESS;
}
