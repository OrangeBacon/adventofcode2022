use std::cmp::Reverse;

use itertools::Itertools;
use libaoc::Timer;

enum Operation {
    Mul(i64),
    Add(i64),
    Square,
}

struct Monkey {
    starting: Vec<i64>,
    operation: Operation,
    divisible: i64,
    yes: usize,
    no: usize,
}

pub fn solve(timer: &mut Timer, _input: &str) -> () {
    let mut _input = [
        Monkey {
            starting: vec![79, 98],
            operation: Operation::Mul(19),
            divisible: 23,
            yes: 2,
            no: 3,
        },
        Monkey {
            starting: vec![54, 65, 75, 74],
            operation: Operation::Add(6),
            divisible: 19,
            yes: 2,
            no: 0,
        },
        Monkey {
            starting: vec![79, 60, 97],
            operation: Operation::Square,
            divisible: 13,
            yes: 1,
            no: 3,
        },
        Monkey {
            starting: vec![74],
            operation: Operation::Add(3),
            divisible: 17,
            yes: 0,
            no: 1,
        },
    ];
    let mut input = [
        Monkey {
            starting: vec![52, 78, 79, 63, 51, 94],
            operation: Operation::Mul(13),
            divisible: 5,
            yes: 1,
            no: 6,
        },
        Monkey {
            starting: vec![77, 94, 70, 83, 53],
            operation: Operation::Add(3),
            divisible: 7,
            yes: 5,
            no: 3,
        },
        Monkey {
            starting: vec![98, 50, 76],
            operation: Operation::Square,
            divisible: 13,
            yes: 0,
            no: 6,
        },
        Monkey {
            starting: vec![92, 91, 61, 75, 99, 63, 84, 69],
            operation: Operation::Add(5),
            divisible: 11,
            yes: 5,
            no: 7,
        },
        Monkey {
            starting: vec![51, 53, 83, 52],
            operation: Operation::Add(7),
            divisible: 3,
            yes: 2,
            no: 0,
        },
        Monkey {
            starting: vec![76, 76],
            operation: Operation::Add(4),
            divisible: 2,
            yes: 4,
            no: 7,
        },
        Monkey {
            starting: vec![75, 59, 93, 69, 76, 96, 65],
            operation: Operation::Mul(19),
            divisible: 17,
            yes: 1,
            no: 3,
        },
        Monkey {
            starting: vec![89],
            operation: Operation::Add(2),
            divisible: 19,
            yes: 2,
            no: 4,
        },
    ];

    timer.lap("Parse");

    let max: i64 = input.iter().map(|x| &x.divisible).product();

    let mut inspections = vec![0; input.len()];
    for _ in 0..10000 {
        for monkey in 0..input.len() {
            inspections[monkey] += input[monkey].starting.len();
            while let Some(mut worry) = input[monkey].starting.pop() {
                match &input[monkey].operation {
                    Operation::Mul(a) => worry *= a,
                    Operation::Add(a) => worry += a,
                    Operation::Square => worry = worry.pow(2),
                }
                worry %= &max;
                let to = if &worry % &input[monkey].divisible == 0 {
                    input[monkey].yes
                } else {
                    input[monkey].no
                };

                input[to].starting.push(worry);
            }
        }
    }
    println!("{inspections:?}");
    let part_1: usize = inspections
        .into_iter()
        .map(|x| Reverse(x))
        .k_smallest(2)
        .map(|x| x.0)
        .product();
    timer.lap("Part 1");

    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    // println!("Part 2: {part_2}");
}
