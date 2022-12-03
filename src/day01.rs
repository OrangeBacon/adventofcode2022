use itertools::Itertools;
use libaoc::Timer;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let mut data = input
        .lines()
        .batching(|it| {
            let mut sum = 0;
            while let Some(num) = it.next() {
                if num.is_empty() {
                    return Some(sum);
                }
                sum += num.parse::<u64>().unwrap();
            }
            None
        })
        .collect_vec();

    timer.lap("Parse");

    data.sort_unstable_by(|a, b| b.cmp(a));
    let a = data[0];
    timer.lap("Part 1");

    let b: u64 = data[0..3].iter().sum();
    timer.lap("Part 2");

    println!("Part 1: {a}");
    println!("Part 2: {b}");
}
