use std::{cmp::Ordering, collections::VecDeque};

use itertools::Itertools;
use libaoc::Timer;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Num(i32),
    List(Vec<Value>),
}

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let mut data = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|chunk| parse_list(&mut chunk.chars().collect()))
        .collect_vec();
    timer.lap("Parse");

    let part_1: usize = data
        .chunks(2)
        .enumerate()
        .map(|(idx, chunk)| match compare_list(&chunk[0], &chunk[1]) {
            Ordering::Less => idx + 1,
            Ordering::Equal => todo!(),
            Ordering::Greater => 0,
        })
        .sum();
    timer.lap("Part 1");

    let a = Value::List(vec![Value::List(vec![Value::Num(2)])]);
    let b = Value::List(vec![Value::List(vec![Value::Num(6)])]);
    data.push(a.clone());
    data.push(b.clone());
    data.sort_by(compare_list);
    let a = data.iter().position(|x| x == &a).unwrap() + 1;
    let b = data.iter().position(|x| x == &b).unwrap() + 1;
    let part_2 = a * b;
    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn parse_list(input: &mut VecDeque<char>) -> Value {
    if input[0] == '[' {
        input.pop_front();
        let mut data = vec![];

        loop {
            if input[0] == ']' {
                input.pop_front();
                break;
            } else if input[0] == ',' {
                input.pop_front();
            }

            data.push(parse_list(input));
        }

        return Value::List(data);
    }

    let mut val = 0;

    while let Some(&peek) = input.get(0) {
        if !peek.is_ascii_digit() {
            break;
        }
        input.pop_front();
        val *= 10;
        val += peek.to_digit(10).unwrap() as i32;
    }

    Value::Num(val)
}

fn compare_list(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Num(a), Value::Num(b)) => a.cmp(b),
        (Value::List(a), Value::List(b)) => {
            let mut a = a.iter();
            let mut b = b.iter();
            loop {
                match (a.next(), b.next()) {
                    (Some(a), Some(b)) => {
                        let ord = compare_list(a, b);
                        if ord.is_ne() {
                            break ord;
                        }
                    }
                    (None, None) => break Ordering::Equal,
                    (None, _) => break Ordering::Less,
                    (_, None) => break Ordering::Greater,
                }
            }
        }
        (Value::List(_), Value::Num(b)) => compare_list(a, &Value::List(vec![Value::Num(*b)])),
        (Value::Num(a), Value::List(_)) => compare_list(&Value::List(vec![Value::Num(*a)]), b),
    }
}
