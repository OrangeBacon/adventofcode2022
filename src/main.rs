use libaoc::Timer;

mod day01;

fn main() {
    let mut timer = Timer::now();
    day01::solve(&mut timer, "abc");

    println!("{}", timer);
}
