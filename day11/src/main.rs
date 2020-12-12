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

    println!("PART 1 | {}", noccupied(&simulate(&data, 4, 1)));

    println!(
        "PART 2 | {}",
        noccupied(&simulate(&data, 5, std::isize::MAX))
    );
}
