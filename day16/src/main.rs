use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let data: Vec<String> = read_lines("../data/16_input")
        .unwrap()
        .map(|s| s.unwrap())
        .collect();

    println!("{:?}", data);
}
