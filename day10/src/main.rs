use aoc_utils::*;

mod lib;

use lib::*;

fn main() {
    let mut data: Vec<usize> = read_lines("../data/10_input")
        .unwrap()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    prepare(&mut data);

    let counts = count_jolt_differences(&data);

    println!("PART 1 | {}", counts[0] * counts[2]);
    println!("PART 2 | {}", n_arrangements(&data));
}
