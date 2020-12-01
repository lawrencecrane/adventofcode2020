use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::path::Path;

mod matching;

use matching::{find_matching_sum, find_matching_sum_of_three, find_matching_sum_of_two};

fn main() {
    let numbers: Vec<u32> = read_lines("./data/input").unwrap().map(to_number).collect();

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

fn to_number(x: std::result::Result<String, std::io::Error>) -> u32 {
    x.unwrap().parse::<u32>().unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
