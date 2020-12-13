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

    println!("{:?}", find_earliest(&timetable));
}
