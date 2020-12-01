use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::path::Path;

fn main() {
    let lines = read_lines("./data/input").unwrap();

    let (a, b) = find_matching_sum(lines, &to_number, 2020).unwrap();

    println!("{} and {}, answer: {}", a, b, a * b);
}

fn find_matching_sum<I>(
    numbers: I,
    to_number: &dyn Fn(<I as Iterator>::Item) -> u32,
    sum: u32,
) -> Option<(u32, u32)>
where
    I: Iterator,
{
    let (_, found_sum) =
        numbers.fold(
            (Vec::new(), None),
            |(mut numbers, previous_sum), x| match previous_sum {
                Some(_) => (numbers, previous_sum),
                None => {
                    let n = to_number(x);
                    let sum = _find_matching_sum(&numbers, &n, sum);
                    numbers.push(n);

                    (numbers, sum)
                }
            },
        );

    found_sum
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
    fn to_number<'a>(x: &'a u32) -> u32 {
        *x
    }

    #[test]
    fn find_matching_sum_test() {
        let numbers: Vec<u32> = Vec::from([1721, 979, 366, 299, 675, 1456]);

        let (a, b) = super::find_matching_sum(numbers.iter(), &to_number, 2020).unwrap();

        assert_eq!(a * b, 514579);
    }
}
