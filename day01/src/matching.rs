use std::iter::Iterator;

pub fn find_matching_sum<'a, I, T>(
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

pub fn find_matching_sum_of_three(
    numbers: &Vec<u32>,
    number: &u32,
    sum: u32,
) -> Option<(u32, u32, u32)> {
    find_matching_sum(numbers.iter(), sum, &|xs, x, _| {
        find_matching_sum_of_two(xs, &(number + x), sum).and_then(|(a, _)| Some((a, *x, *number)))
    })
}

pub fn find_matching_sum_of_two(xs: &Vec<u32>, n: &u32, sum: u32) -> Option<(u32, u32)> {
    xs.iter()
        .find(|x| *x + n == sum)
        .and_then(|x| Some((*x, *n)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_matching_sum_of_two_test() {
        let numbers: Vec<u32> = Vec::from([1721, 979, 366, 299, 675, 1456]);

        let (a, b) =
            super::find_matching_sum(numbers.iter(), 2020, &super::find_matching_sum_of_two)
                .unwrap();

        assert_eq!(a * b, 514579);
    }

    #[test]
    fn find_matching_sum_of_three_test() {
        let numbers: Vec<u32> = Vec::from([1721, 979, 366, 299, 675, 1456]);

        let (a, b, c) =
            super::find_matching_sum(numbers.iter(), 2020, &super::find_matching_sum_of_three)
                .unwrap();

        assert_eq!(a * b * c, 241861950);
    }
}
