use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn to_number(x: std::result::Result<String, std::io::Error>) -> u32 {
    x.unwrap().parse::<u32>().unwrap()
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
