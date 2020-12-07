use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let bags = to_bags(read_lines("../data/07_input").unwrap().map(|s| s.unwrap()));

    println!(
        "PART 1 | {}",
        n_bags_containing(&bags, "shiny gold".to_string())
    );

    println!(
        "PART 2 | {}",
        n_bag_contains(&bags, "shiny gold".to_string())
    );
}
