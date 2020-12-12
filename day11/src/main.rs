use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let data = to_layout(
        &read_lines("../data/11_input")
            .unwrap()
            .map(|s| s.unwrap())
            .collect(),
    );

    println!(
        "PART 1 | {}",
        noccupied(&simulate_with_immediate_adjacent_seats(&data))
    );
}
