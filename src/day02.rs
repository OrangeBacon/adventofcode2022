use itertools::Itertools;
use libaoc::Timer;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let data = std::fs::read_to_string("./data/day02.txt").unwrap();
    let data = data
        .lines()
        .map(|l| l.split_whitespace().collect_vec())
        .collect_vec();

    timer.lap("Parse");

    let mut score = 0;
    for line in &data {
        score += match line[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("Invalid data"),
        };
        score += match (line[0], line[1]) {
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            _ => 0,
        };
    }
    timer.lap("Part 1");

    let mut score2 = 0;
    for line in &data {
        let my_move = match line[1] {
            "Y" => match line[0] {
                "A" => "X",
                "B" => "Y",
                "C" => "Z",
                _ => panic!(),
            },
            "X" => match line[0] {
                "A" => "Z",
                "B" => "X",
                "C" => "Y",
                _ => panic!(),
            },
            "Z" => match line[0] {
                "A" => "Y",
                "B" => "Z",
                "C" => "X",
                _ => panic!(),
            },
            _ => panic!(),
        };
        score2 += match my_move {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("Invalid data"),
        };
        score2 += match (line[0], my_move) {
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            _ => 0,
        };
    }
    timer.lap("Part 2");

    println!("Part 1: {score}");
    println!("Part 2: {score2}");
}
