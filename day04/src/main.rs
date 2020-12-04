use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let map: Vec<String> = read_lines("../data/04_input")
        .unwrap()
        .map(|s| s.unwrap())
        .collect();
}
