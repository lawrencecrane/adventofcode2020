use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let seat_ids: Vec<usize> = read_lines("../data/05_input")
        .unwrap()
        .map(|s| get_seat_id(identify_seat(&s.unwrap())))
        .collect();

    let max_id = seat_ids.iter().max().unwrap();

    println!("PART 1 | max seat id: {}", max_id);
}
