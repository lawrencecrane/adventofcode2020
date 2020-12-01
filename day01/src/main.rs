use aoc_utils::{read_lines, to_number};
use std::iter::Iterator;

mod matching;

use matching::{find_matching_sum, find_matching_sum_of_three, find_matching_sum_of_two};

fn main() {
    let numbers: Vec<u32> = read_lines("../data/input")
        .unwrap()
        .map(to_number)
        .collect();

    solve_part_one(&numbers);
    solve_part_two(&numbers);
}

fn solve_part_one(numbers: &Vec<u32>) {
    let (a, b) = find_matching_sum(numbers.iter(), 2020, &find_matching_sum_of_two).unwrap();

    println!("PART 1: {} and {}, answer: {}", a, b, a * b);
}

fn solve_part_two(numbers: &Vec<u32>) {
    let (a, b, c) = find_matching_sum(numbers.iter(), 2020, &find_matching_sum_of_three).unwrap();

    println!("PART 2: {}, {} and {}, answer: {}", a, b, c, a * b * c);
}
