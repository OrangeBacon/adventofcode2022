use std::collections::HashSet;

use itertools::Itertools;
use libaoc::Timer;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let input = input.chars().collect_vec();
    timer.lap("Parse");

    let part_1 = window(&input, 4);
    timer.lap("Part 1");

    let part_2 = window(&input, 14);
    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    println!("Part 1: {part_2}");
}

fn window(arr: &[char], len: usize) -> usize {
    let mut ret = len;
    for a in arr.windows(len) {
        if HashSet::<char>::from_iter(a.iter().copied()).len() == len {
            break;
        }
        ret += 1;
    }

    ret
}
