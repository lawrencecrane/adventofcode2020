use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let passwords = to_passwords(read_lines("../data/02_input").unwrap().map(|x| x.unwrap()));

    println!("{}", n_valid_passwords(&passwords));
}
