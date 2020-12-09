use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let code = to_codes(read_lines("../data/08_input").unwrap().map(|s| s.unwrap()));

    println!("PART 1 | {}", execute(&code).0);
    println!("PART 2 | {}", execute_corrupted_program(&code));
}
