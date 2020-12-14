use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let program = to_program(
        read_lines("../data/14_input")
            .unwrap()
            .map(|s| s.unwrap().parse().unwrap())
            .collect(),
    );

    println!("PART 1 | {}", run(&program));
}
