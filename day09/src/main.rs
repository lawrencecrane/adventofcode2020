use aoc_utils::*;

mod lib;

use day09::find_contiguous_set_summing_to;
use lib::*;

fn main() {
    let data: Vec<usize> = read_lines("../data/09_input")
        .unwrap()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    let mismatching = find_first_mismatch(&data, 25);

    let set = find_contiguous_set_summing_to(mismatching, &data);

    println!("PART 1 | {}", &mismatching);

    println!(
        "PART 2 | {}",
        set.iter().min().unwrap() + set.iter().max().unwrap()
    );
}
