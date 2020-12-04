extern crate lazy_static;

use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let passports = to_passports(read_lines("../data/04_input").unwrap().map(|s| s.unwrap()));

    println!(
        "PART 1 | number of valid passports: {}",
        n_valid_keys(&passports)
    );

    println!(
        "PART 2 | number of valid passports: {}",
        n_valid_keys_and_values(&passports)
    );
}
