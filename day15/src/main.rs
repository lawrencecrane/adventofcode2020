mod lib;

use lib::*;

fn main() {
    println!(
        "PART 1 | {}",
        play_memory_game(vec![6, 13, 1, 15, 2, 0], 2020)
    );
}
