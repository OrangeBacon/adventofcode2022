use libaoc::Timer;
use regex::Regex;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let mut lines = input.lines();

    let mut crates = vec![vec![]; 10];
    let mut moves: Vec<(usize, usize, usize)> = vec![];

    'a: for mut line in &mut lines {
        let mut col = 0;
        loop {
            if line.starts_with("[") {
                crates[col].push(&line[1..2]);
            } else if line.starts_with(" 1") {
                break 'a;
            }
            col += 1;
            line = match line.get(4..) {
                Some(l) => l,
                None => break,
            }
        }
    }
    lines.next(); // skip blank line

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        let cap = re.captures(line).unwrap();
        moves.push((
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        ));
    }

    for col in &mut crates {
        col.reverse();
    }

    timer.lap("Parse");

    let mut part_1 = crates.clone();
    for &(count, from, to) in &moves {
        for _ in 0..count {
            let val = part_1[from - 1].pop().unwrap();
            part_1[to - 1].push(val);
        }
    }
    let part_1: String = part_1
        .iter()
        .filter(|x| !x.is_empty())
        .map(|col| col[col.len() - 1])
        .collect();
    timer.lap("Part 1");

    for &(count, from, to) in &moves {
        let mut vals = vec![];
        for _ in 0..count {
            let val = crates[from - 1].pop().unwrap();
            vals.push(val);
        }
        vals.reverse();
        for val in vals {
            crates[to - 1].push(val);
        }
    }
    let part_2: String = crates
        .iter()
        .filter(|x| !x.is_empty())
        .map(|col| col[col.len() - 1])
        .collect();
    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    println!("Part 1: {part_2}");
}
