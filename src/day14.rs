use std::collections::HashMap;

use itertools::Itertools;
use libaoc::Timer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Sand,
    Rock,
}

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let mut map = HashMap::new();
    for line in input.lines() {
        for (start, end) in line.split(" -> ").tuple_windows() {
            let (sx, sy) = start
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            let (ex, ey) = end
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            let horiz = sx.min(ex)..=sx.max(ex);
            let vert = sy.min(ey)..=sy.max(ey);
            for x in horiz {
                for y in vert.clone() {
                    map.insert((x, y), Cell::Rock);
                }
            }
        }
    }
    let map2 = map.clone();
    timer.lap("Parse");

    let lowest = map.iter().map(|((_, y), _)| *y).max().unwrap();
    'out: loop {
        let mut sand = (500, 0);
        loop {
            if sand.1 > lowest {
                break 'out;
            }
            let a = *map.get(&(sand.0, sand.1 + 1)).unwrap_or(&Cell::Empty);
            let b = *map.get(&(sand.0 - 1, sand.1 + 1)).unwrap_or(&Cell::Empty);
            let c = *map.get(&(sand.0 + 1, sand.1 + 1)).unwrap_or(&Cell::Empty);
            if a == Cell::Empty {
                sand.1 += 1;
            } else if b == Cell::Empty {
                sand.1 += 1;
                sand.0 -= 1;
            } else if c == Cell::Empty {
                sand.1 += 1;
                sand.0 += 1;
            } else {
                break;
            }
        }
        map.insert(sand, Cell::Sand);
    }
    let part_1 = map.iter().filter(|&(_, &c)| c == Cell::Sand).count();
    timer.lap("Part 1");

    let mut map = map2;
    loop {
        let mut sand = (500, 0);
        loop {
            let default = if sand.1 > lowest {
                &Cell::Rock
            } else {
                &Cell::Empty
            };
            let a = *map.get(&(sand.0, sand.1 + 1)).unwrap_or(default);
            let b = *map.get(&(sand.0 - 1, sand.1 + 1)).unwrap_or(default);
            let c = *map.get(&(sand.0 + 1, sand.1 + 1)).unwrap_or(default);
            if a == Cell::Empty {
                sand.1 += 1;
            } else if b == Cell::Empty {
                sand.1 += 1;
                sand.0 -= 1;
            } else if c == Cell::Empty {
                sand.1 += 1;
                sand.0 += 1;
            } else {
                break;
            }
        }
        map.insert(sand, Cell::Sand);
        if sand == (500, 0) {
            break;
        }
    }
    let part_2 = map.iter().filter(|&(_, &c)| c == Cell::Sand).count();
    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
