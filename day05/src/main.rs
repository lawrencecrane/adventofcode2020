use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let max_id = read_lines("../data/05_input")
        .unwrap()
        .map(|s| get_seat_id(identify_seat(&s.unwrap())))
        .max()
        .unwrap();

    println!("PART 1 | max seat id: {}", max_id);
}
