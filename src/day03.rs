use itertools::Itertools;
use libaoc::Timer;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let mut a = 0;
    for (left, right) in input.lines().map(|l| l.split_at(l.len() / 2)) {
        for ch in left.chars() {
            if right.contains(ch) {
                a += priority(ch);
                break;
            }
        }
    }
    timer.lap("Part 1");

    let mut b = 0;
    for bags in &input.lines().chunks(3) {
        let bags = bags.collect_vec();
        for ch in ('a'..='z').chain('A'..='Z') {
            if bags.iter().all(|bag| bag.contains(ch)) {
                b += priority(ch);
                break;
            }
        }
    }
    timer.lap("Part 2");

    println!("Part 1: {a}");
    println!("Part 2: {b}");
}

fn priority(ch: char) -> u32 {
    if matches!(ch, 'a'..='z') {
        ch as u32 - 'a' as u32 + 1
    } else {
        ch as u32 - 'A' as u32 + 27
    }
}
