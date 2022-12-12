use std::collections::HashMap;

use itertools::Itertools;
use libaoc::Timer;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let data = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        'a'
                    }
                    'E' => {
                        end = (x, y);
                        'z'
                    }
                    _ => c,
                } as u32 - 'a' as u32)
                .collect_vec()
        })
        .collect_vec();
    timer.lap("Parse");

    let part_1 = distance(&data, start, end, usize::MAX).unwrap();
    timer.lap("Part 1");

    let mut part_2 = usize::MAX;
    for (y, row) in data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val == 0 && shortest_dist((x, y), end) < part_2 {
                if let Some(new) = distance(&data, (x, y), end, part_2) {
                    part_2 = part_2.min(new);
                }
            }
        }
    }
    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn shortest_dist(a: (usize, usize), b: (usize, usize)) -> usize {
    let x = a.0.max(b.0) - a.0.min(b.0);
    let y = a.1.max(b.1) - a.1.min(b.1);
    x + y
}

fn distance(
    data: &[Vec<u32>],
    start: (usize, usize),
    end: (usize, usize),
    max: usize,
) -> Option<usize> {
    let mut locations = HashMap::new();
    locations.insert(start, 0);
    loop {
        let mut next_iter = HashMap::new();
        let mut min_steps = usize::MAX;
        for ((x, y), mut steps) in locations {
            steps += 1;
            min_steps = min_steps.min(steps);

            let current = data[y][x];
            if let Some(a) = data.get(y).and_then(|v| Some(*v.get(x.checked_add(1)?)?)) {
                if a <= current + 1 {
                    next_iter
                        .entry((x + 1, y))
                        .and_modify(|v| *v = steps.min(*v))
                        .or_insert(steps);
                }
            }

            if let Some(b) = data.get(y).and_then(|v| Some(*v.get(x.checked_sub(1)?)?)) {
                if b <= current + 1 {
                    next_iter
                        .entry((x - 1, y))
                        .and_modify(|v| *v = steps.min(*v))
                        .or_insert(steps);
                }
            }

            if let Some(c) = y
                .checked_add(1)
                .and_then(|y| data.get(y))
                .and_then(|v| v.get(x).copied())
            {
                if c <= current + 1 {
                    next_iter
                        .entry((x, y + 1))
                        .and_modify(|v| *v = steps.min(*v))
                        .or_insert(steps);
                }
            }

            if let Some(d) = y
                .checked_sub(1)
                .and_then(|y| data.get(y))
                .and_then(|v| v.get(x).copied())
            {
                if d <= current + 1 {
                    next_iter
                        .entry((x, y - 1))
                        .and_modify(|v| *v = steps.min(*v))
                        .or_insert(steps);
                }
            }
        }
        locations = next_iter;

        if let Some(val) = locations.get(&end) {
            break Some(*val);
        }

        if min_steps >= max {
            break None;
        }
    }
}
