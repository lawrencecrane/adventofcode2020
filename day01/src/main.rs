use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::path::Path;

fn main() {
    let numbers: Vec<u32> = read_lines("./data/input").unwrap().map(to_number).collect();

    solve_part_one(&numbers);
    solve_part_two(&numbers);
}

fn solve_part_one(numbers: &Vec<u32>) {
    let (a, b) = find_matching_sum(numbers.iter(), 2020, &_find_matching_sum).unwrap();

    println!("PART 1: {} and {}, answer: {}", a, b, a * b);
}

fn solve_part_two(numbers: &Vec<u32>) {
    let (a, b, c) = find_matching_sum(numbers.iter(), 2020, &_find_matching_sum_of_three).unwrap();

    println!("PART 2: {}, {} and {}, answer: {}", a, b, c, a * b * c);
}

fn find_matching_sum<'a, I, T>(
    numbers: I,
    sum: u32,
    finder: &dyn Fn(&Vec<u32>, &u32, u32) -> Option<T>,
) -> Option<T>
where
    I: Iterator<Item = &'a u32>,
{
    let (_, matching) = numbers.fold((Vec::new(), None), |(mut xs, matching), x| match matching {
        Some(_) => (xs, matching),
        None => {
            let sum = finder(&xs, &x, sum);
            xs.push(*x);

            (xs, sum)
        }
    });

    matching
}

fn _find_matching_sum_of_three(
    numbers: &Vec<u32>,
    number: &u32,
    sum: u32,
) -> Option<(u32, u32, u32)> {
    find_matching_sum(numbers.iter(), sum, &|xs, x, _| {
        _find_matching_sum(xs, &(number + x), sum).and_then(|(a, _)| Some((a, *x, *number)))
    })
}

fn _find_matching_sum(xs: &Vec<u32>, n: &u32, sum: u32) -> Option<(u32, u32)> {
    xs.iter()
        .find(|x| *x + n == sum)
        .and_then(|x| Some((*x, *n)))
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

#[cfg(test)]
mod tests {
    #[test]
    fn find_matching_sum_test() {
        let numbers: Vec<u32> = Vec::from([1721, 979, 366, 299, 675, 1456]);

        let (a, b) =
            super::find_matching_sum(numbers.iter(), 2020, &super::_find_matching_sum).unwrap();

        assert_eq!(a * b, 514579);
    }

    #[test]
    fn find_matching_sum_of_three_test() {
        let numbers: Vec<u32> = Vec::from([1721, 979, 366, 299, 675, 1456]);

        let (a, b, c) =
            super::find_matching_sum(numbers.iter(), 2020, &super::_find_matching_sum_of_three)
                .unwrap();

        assert_eq!(a * b * c, 241861950);
    }
}
