use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let bags = to_bags(read_lines("../data/07_input").unwrap().map(|s| s.unwrap()));

    println!(
        "PART 1 | {}",
        n_bags_that_can_contain(&bags, "shiny gold".to_string())
    );
}
