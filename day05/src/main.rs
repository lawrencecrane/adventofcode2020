use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let seats: Vec<(usize, usize)> = read_lines("../data/05_input")
        .unwrap()
        .map(|s| identify_seat(&s.unwrap()))
        .collect();

    println!("{:?}", seats);
}
