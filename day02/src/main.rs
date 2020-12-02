use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let passwords = to_passwords(read_lines("../data/02_input").unwrap().map(|x| x.unwrap()));

    println!("PART 1: {}", n_valid_passwords_by_count_policy(&passwords));

    println!(
        "PART 2: {}",
        n_valid_passwords_by_position_policy(&passwords)
    );
}
