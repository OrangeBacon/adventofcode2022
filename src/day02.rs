use itertools::Itertools;
use libaoc::Timer;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
use Move::*;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let data = input
        .lines()
        .map(|l| l.split_whitespace().map(Move::from_str).collect_vec())
        .collect_vec();

    timer.lap("Parse");

    let mut score = 0;
    for line in &data {
        score += line[1] as i32;
        score += line[0].win_score(line[1]);
    }
    timer.lap("Part 1");

    let mut score2 = 0;
    for line in &data {
        let my_move = line[1].to_should_win(line[0]);
        score2 += my_move as i32;
        score2 += line[0].win_score(my_move);
    }
    timer.lap("Part 2");

    println!("Part 1: {score}");
    println!("Part 2: {score2}");
}

impl Move {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Invalid data"),
        }
    }

    fn to_should_win(&self, other: Self) -> Self {
        match (self, other) {
            (Paper, _) => other,
            (Rock, Rock) => Scissors,
            (Rock, Paper) => Rock,
            (Rock, Scissors) => Paper,
            (Scissors, Rock) => Paper,
            (Scissors, Paper) => Scissors,
            (Scissors, Scissors) => Rock,
        }
    }

    fn win_score(&self, other: Self) -> i32 {
        match (self, other) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            _ => 0,
        }
    }
}
