use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let passports = to_passports(read_lines("../data/04_input").unwrap().map(|s| s.unwrap()));

    println!(
        "PART 1 | number of valid passports: {}",
        n_valid(&passports)
    );
}
