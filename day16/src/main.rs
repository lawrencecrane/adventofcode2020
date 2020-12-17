use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let data = to_ticket_data(
        &read_lines("../data/16_input")
            .unwrap()
            .map(|s| s.unwrap())
            .collect(),
    );

    println!("{:?}", data);
}
