use itertools::Itertools;
use libaoc::Timer;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let data = input
        .lines()
        .map(|x| {
            x.split([',', '-'])
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    timer.lap("Parse");

    let part_1 = data.iter().fold(0, |acc, val| {
        let a = val[0]..=val[1];
        let b = val[2]..=val[3];
        if (a.start() <= b.start() && a.end() >= b.end())
            || (b.start() <= a.start() && b.end() >= a.end())
        {
            acc + 1
        } else {
            acc
        }
    });
    timer.lap("Part 1");

    let part_2 = data.iter().fold(0, |acc, val| {
        let a = val[0]..=val[1];
        let b = val[2]..=val[3];
        if a.contains(b.start())
            || a.contains(b.end())
            || b.contains(a.start())
            || b.contains(a.end())
        {
            acc + 1
        } else {
            acc
        }
    });
    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    println!("Part 1: {part_2}");
}
