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

    {
        let endpoint = travel(&instructions);
        println!("PART 1 | {}", endpoint.0.abs() + endpoint.1.abs());
    }

    {
        let endpoint = travel_with_waypoint(&instructions);
        println!("PART 2 | {}", endpoint.0.abs() + endpoint.1.abs());
    }
}
