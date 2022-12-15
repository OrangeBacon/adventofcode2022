use std::collections::HashSet;

use libaoc::Timer;
use regex::Regex;

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let mut map: Vec<((i64, i64), (i64, i64))> = vec![];
    let re = Regex::new(
        r"Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)",
    )
    .unwrap();
    for line in re.captures_iter(input) {
        map.push((
            (line[1].parse().unwrap(), line[2].parse().unwrap()),
            (line[3].parse().unwrap(), line[4].parse().unwrap()),
        ));
    }

    let beacons: HashSet<_> = map.iter().map(|(_, a)| *a).collect();
    timer.lap("Parse");

    let part_1 = count_used(&map, 2000000, i64::MIN, i64::MAX).0;
    timer.lap("Part 1");

    let max = 4000000;
    let mut part_2 = (0, 0);
    for row in 0..=max {
        let used = count_used(&map, row, 0, max);
        if used.0 != (max + 1) as _ {
            part_2 = (used.1, row);
            if !beacons.contains(&part_2) {
                break;
            }
        }
    }
    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    println!("Part 2: {}", part_2.0 * 4000000 + part_2.1);
}

fn count_used(map: &[((i64, i64), (i64, i64))], row: i64, min: i64, max: i64) -> (usize, i64) {
    let mut ranges = vec![];
    for (sensor, beacon) in map {
        let dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let dist = dist - (sensor.1 - row).abs();
        if beacon.1 == row {
            if beacon.0 < sensor.0 {
                ranges.push(sensor.0 - dist + 1..=sensor.0 + dist);
            } else {
                ranges.push(sensor.0 - dist..=sensor.0 + dist - 1);
            }
        } else if dist > 0 {
            ranges.push(sensor.0 - dist..=sensor.0 + dist);
        }
    }

    ranges.sort_unstable_by(|a, b| a.start().cmp(b.start()));
    let mut reduced = vec![ranges[0].clone()];
    for range in ranges {
        let last = reduced.last_mut().unwrap();
        if last.end() + 1 >= *range.start() {
            let start = *last.start().min(range.start());
            let end = *last.end().max(range.end());
            *last = start..=end;
        } else {
            reduced.push(range);
        }
    }

    let mut count = 0;
    for range in &reduced {
        let min = *range.start().max(&min);
        let max = *range.end().min(&max);
        count += (min..=max).count()
    }

    if reduced.len() == 2 {
        (count, reduced[0].end() + 1)
    } else {
        (count, 0)
    }
}
