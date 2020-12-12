use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let adapters = Adapters::new(
        read_lines("../data/10_input")
            .unwrap()
            .map(|s| s.unwrap().parse().unwrap())
            .collect(),
    );

    let counts = adapters.count_jolt_differences();

    println!("PART 1 | {}", counts[0] * counts[2]);
    println!("PART 2 | {}", adapters.n_arrangements());
}
