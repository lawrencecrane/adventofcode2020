use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let data: Vec<usize> = read_lines("../data/10_input")
        .unwrap()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    println!("{:?}", count_jolt_differences(&data));
}
