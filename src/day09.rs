use std::collections::HashSet;

use itertools::Itertools;
use libaoc::Timer;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let commands = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, dist)| (dir, dist.parse::<i32>().unwrap()))
        .collect_vec();
    timer.lap("Parse");

    let part_1 = move_rope(&commands, &mut [(0, 0); 2]);
    timer.lap("Part 1");

    let part_2 = move_rope(&commands, &mut [(0, 0); 100]);
    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn move_rope(commands: &[(&str, i32)], rope: &mut [(i32, i32)]) -> usize {
    let mut visited = HashSet::from([*rope.last().unwrap()]);
    for command in commands {
        for _ in 0..command.1 {
            match command.0 {
                "R" => rope[0].0 += 1,
                "U" => rope[0].1 += 1,
                "L" => rope[0].0 -= 1,
                "D" => rope[0].1 -= 1,
                _ => panic!(),
            }
            move_knots(rope);
            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn move_knots(rope: &mut [(i32, i32)]) -> (i32, i32) {
    let mut tail = (0, 0);
    for idx in 0..rope.len() - 1 {
        tail = move_tail(rope[idx], rope[idx + 1]);
        rope[idx + 1] = tail;
    }

    tail
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head.1 == tail.1 {
        if head.0 >= tail.0 + 2 {
            return (head.0 - 1, tail.1);
        } else if head.0 <= tail.0 - 2 {
            return (head.0 + 1, tail.1);
        }
    } else if head.0 == tail.0 {
        if head.1 >= tail.1 + 2 {
            return (head.0, tail.1 + 1);
        } else if head.1 <= tail.1 - 2 {
            return (head.0, tail.1 - 1);
        }
    } else if (head.0 - tail.0).abs() == (head.1 - tail.1).abs() {
        let x = if head.0 > tail.0 { -1 } else { 1 };
        let y = if head.1 > tail.1 { -1 } else { 1 };
        return (head.0 + x, head.1 + y);
    } else if (head.0 - tail.0).abs() > 1 {
        if head.0 > tail.0 {
            return (head.0 - 1, head.1);
        } else {
            return (head.0 + 1, head.1);
        }
    } else if (head.1 - tail.1).abs() > 1 {
        if head.1 > tail.1 {
            return (head.0, head.1 - 1);
        } else {
            return (head.0, head.1 + 1);
        }
    }

    tail
}
