use libaoc::Timer;

#[derive(Clone, Copy)]
enum Instruction {
    Add(i32),
    Noop,
}

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let mut instructions = vec![];
    for line in input.lines() {
        if line == "noop" {
            instructions.push(Instruction::Noop);
        } else if let Some(("addx", val)) = line.split_once(' ') {
            instructions.push(Instruction::Noop);
            instructions.push(Instruction::Noop);
            instructions.push(Instruction::Add(val.parse().unwrap()));
        }
    }
    timer.lap("Parse");

    let mut x = 1;
    let mut part_1 = 0;
    let mut cycles = 0;
    for instruction in &instructions {
        match instruction {
            Instruction::Add(val) => {
                x += *val;
                continue;
            }
            Instruction::Noop => (),
        }

        let pos = cycles % 40;
        if (x - 1..=x + 1).contains(&pos) {
            print!("#");
        } else {
            print!(" ");
        }
        if (cycles + 1) % 40 == 0 {
            println!("");
        }

        cycles += 1;
        if [20, 60, 100, 140, 180, 220].contains(&cycles) {
            part_1 += cycles * x;
        }
    }
    timer.lap("Part 1");

    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    // println!("Part 2: {part_2}");
}
