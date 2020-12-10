use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let mut data: Vec<usize> = read_lines("../data/10_input")
        .unwrap()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    let counts = count_jolt_differences(&mut data);

    println!("PART 1 | {}", counts[0] * counts[2]);
}
