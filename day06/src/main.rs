use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let groups = to_groups(read_lines("../data/06_input").unwrap().map(|s| s.unwrap()));

    println!("PART 1 | {}", count_any_yes_answers(&groups));
}
