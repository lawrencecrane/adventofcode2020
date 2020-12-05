use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let input: Vec<String> = read_lines("../data/05_input")
        .unwrap()
        .map(|s| s.unwrap())
        .collect();

    println!("{:?}", input);
}
