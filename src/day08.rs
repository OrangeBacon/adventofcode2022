use std::collections::HashSet;

use itertools::Itertools;
use libaoc::Timer;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let data = input
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() + 1).collect_vec())
        .collect_vec();
    timer.lap("Parse");

    let mut seen = HashSet::new();
    for (y, row) in data.iter().enumerate() {
        let mut current = 0;
        for (x, &tree) in row.iter().enumerate() {
            if tree > current {
                current = tree;
                seen.insert((x, y));
            } else if tree == 10 {
                break;
            }
        }
        current = 0;
        for (x, &tree) in row.iter().enumerate().rev() {
            if tree > current {
                current = tree;
                seen.insert((x, y));
            }
        }
    }
    for x in 0..data[0].len() {
        let mut current = 0;
        for y in 0..data.len() {
            let tree = data[y][x];
            if tree > current {
                current = tree;
                seen.insert((x, y));
            } else if tree == 10 {
                break;
            }
        }
        current = 0;
        for y in (0..data.len()).rev() {
            let tree = data[y][x];
            if tree > current {
                current = tree;
                seen.insert((x, y));
            } else if tree == 10 {
                break;
            }
        }
    }
    let part_1 = seen.len();
    timer.lap("Part 1");

    let mut part_2 = 0;
    for (y, row) in data.iter().enumerate() {
        for (x, &tree) in row.iter().enumerate() {
            let mut score = 1;
            let mut current = 0;
            for x in x + 1..row.len() {
                let Some(&data) = data.get(y).and_then(|row| row.get(x)) else {
                    break;
                };
                current += 1;
                if data >= tree {
                    break;
                }
            }

            score *= current;
            current = 0;
            for x in (0..x).rev() {
                let Some(&data) = data.get(y).and_then(|row| row.get(x)) else {
                    break;
                };
                current += 1;
                if data >= tree {
                    break;
                }
            }

            score *= current;
            current = 0;
            for y in y + 1..data.len() {
                let Some(&data) = data.get(y).and_then(|row| row.get(x)) else {
                    break;
                };
                current += 1;
                if data >= tree {
                    break;
                }
            }

            score *= current;
            current = 0;
            for y in (0..y).rev() {
                let Some(&data) = data.get(y).and_then(|row| row.get(x)) else {
                    break;
                };
                current += 1;
                if data >= tree {
                    break;
                }
            }

            score *= current;
            part_2 = part_2.max(score);
        }
    }
    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
