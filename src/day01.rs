use itertools::Itertools;
use libaoc::Timer;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let data = std::fs::read_to_string("./data/day01.txt").unwrap();
    let mut data = data.lines().batching(|it| {
        let mut sum = 0;
        while let Some(num) = it.next() {
            if num.is_empty() {
                return Some(sum);
            }
            sum += num.parse::<u64>().unwrap();
        }
        None
    }).collect_vec();

    timer.lap("Parse");

    data.sort_unstable_by(|a, b| b.cmp(a));
    let a = data[0];
    timer.lap("Part 1");

    let b = data[0] + data[1] + data[2];
    timer.lap("Part 2");

    println!("Part 1: {a}");
    println!("Part 2: {b}");
}
