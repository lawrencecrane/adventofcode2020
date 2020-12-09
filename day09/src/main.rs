use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let data: Vec<usize> = read_lines("../data/09_input")
        .unwrap()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    println!("PART 1 | {}", find_first_mismatch(data, 25));
}
