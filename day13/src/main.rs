use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let timetable = Timetable::parse(
        read_lines("../data/13_input")
            .unwrap()
            .map(|s| s.unwrap().parse().unwrap())
            .collect(),
    );

    let earliest = find_earliest(&timetable);

    println!(
        "PART 1 | {}",
        earliest.0 * (earliest.1 - timetable.earliest_depart_time)
    );
}
