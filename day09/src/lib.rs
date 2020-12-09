use std::collections::{HashSet, VecDeque};

pub fn find_contiguous_set_summing_to(sum: usize, numbers: &Vec<usize>) -> VecDeque<usize> {
    let (deque, _) = numbers
        .iter()
        .fold((VecDeque::new(), 0), |(mut deque, acc), x| {
            if acc == sum {
                return (deque, acc);
            }

            deque.push_back(*x);

            let mut new_acc = acc + x;

            while new_acc > sum {
                new_acc -= deque.pop_front().unwrap();
            }

            (deque, new_acc)
        });

    deque
}

pub fn find_first_mismatch(numbers: &Vec<usize>, preamble: usize) -> usize {
    let (_, value) = numbers
        .iter()
        .enumerate()
        .skip(preamble)
        .find(|(i, x)| !any_two_sums_to(x, numbers.iter().skip(i - preamble).take(preamble)))
        .unwrap();

    *value
}

fn any_two_sums_to<'a, I>(sum: &usize, mut numbers: I) -> bool
where
    I: Iterator<Item = &'a usize>,
{
    let mut set = HashSet::new();

    loop {
        match numbers.next() {
            Some(x) => {
                if x > sum {
                    continue;
                }

                if set.contains(x) || set.contains(&(sum - x)) {
                    break true;
                }

                set.insert(*x);
                set.insert(sum - x);
            }
            None => break false,
        }
    }
}

#[cfg(test)]
mod tests {
    fn create_factory() -> Vec<usize> {
        vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ]
    }

    #[test]
    fn test_find_first_mismatch() {
        let numbers = create_factory();

        assert_eq!(super::find_first_mismatch(&numbers, 5), 127);
    }

    #[test]
    fn test_find_contiguous_set_summing_to() {
        let numbers = create_factory();

        assert_eq!(
            super::find_contiguous_set_summing_to(127, &numbers),
            vec![15, 25, 47, 40]
        );
    }
}
