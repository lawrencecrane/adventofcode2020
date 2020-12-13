use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let instructions = to_instructions(
        &read_lines("../data/12_input")
            .unwrap()
            .map(|s| s.unwrap().parse().unwrap())
            .collect(),
    );

    println!("{:?}", instructions);
}
