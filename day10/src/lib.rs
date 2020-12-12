use aoc_utils::tribonacci::Tribonacci;

impl Adapters {
    // Add the charging outlet and device's built-in adapter to data and sorts it
    pub fn new(mut data: Vec<usize>) -> Self {
        data.push(0);
        data.push(*data.iter().max().unwrap() + 3);
        data.sort();

        Self { data }
    }

    pub fn count_jolt_differences(&self) -> [usize; 3] {
        count_jolt_differences(self)
    }

    pub fn n_arrangements(&self) -> usize {
        n_arrangements(self)
    }
}

fn n_arrangements(adapters: &Adapters) -> usize {
    let (arrangements, _) = adapters.data.iter().fold(
        (1, 1),
        |(arrangements, nconsecutive), adapter| match adapters.data.contains(&(adapter + 1)) {
            true => (arrangements, nconsecutive + 1),
            false => (
                // From tribonacci sequence, starting: 1, 1, 2, 4... take the nth
                // where nth is the number of consecutive numbers we have seen
                arrangements
                    * Tribonacci {}
                        .into_iter()
                        .filter(|x| *x > 0)
                        .take(nconsecutive)
                        .last()
                        .unwrap(),
                1,
            ),
        },
    );

    arrangements
}

fn count_jolt_differences(adapters: &Adapters) -> [usize; 3] {
    let (_, counts) =
        adapters
            .data
            .iter()
            .skip(1)
            .fold((0, [0, 0, 0]), |(prev, mut counts), adapter| {
                counts[adapter - prev - 1] += 1;

                (*adapter, counts)
            });

    counts
}

pub struct Adapters {
    data: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::Adapters;

    fn create_small_factory() -> Adapters {
        Adapters::new(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4])
    }

    fn create_big_factory() -> Adapters {
        Adapters::new(vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ])
    }

    #[test]
    fn test_count_jolt_differences() {
        assert_eq!(
            super::count_jolt_differences(&create_small_factory()),
            [7, 0, 5]
        );

        assert_eq!(
            super::count_jolt_differences(&create_big_factory()),
            [22, 0, 10]
        );
    }

    #[test]
    fn test_n_arrangements() {
        assert_eq!(super::n_arrangements(&create_small_factory()), 8);
        assert_eq!(super::n_arrangements(&create_big_factory()), 19208);
    }
}
