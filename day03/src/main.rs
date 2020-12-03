use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let map: Vec<String> = read_lines("../data/03_input")
        .unwrap()
        .map(|s| s.unwrap())
        .collect();

    let world = World {
        map: &map,
        position: Coordinate { x: 0, y: 0 },
    };

    println!(
        "PART 1 | encoutered trees: {}",
        traverse_and_count_trees(&world, vec![Coordinate { x: 3, y: 1 }])[0]
    )
}
