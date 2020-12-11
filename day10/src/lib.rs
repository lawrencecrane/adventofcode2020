use aoc_utils::tribonacci::Tribonacci;

// Expects the charging outlet and device's built-in adapter to be part of adapters
// TODO: I don't get why tribonacci sequence works here, research it and comment here why!
pub fn n_arrangements(adapters: &Vec<usize>) -> usize {
    let (arrangements, _) = adapters
        .iter()
        .fold((1, 1), |(arrangements, nrun), adapter| {
            match adapters.contains(&(adapter + 1)) {
                true => (arrangements, nrun + 1),
                false => (
                    arrangements * Tribonacci {}.into_iter().take(nrun + 1).last().unwrap(),
                    1,
                ),
            }
        });

    arrangements
}

// Expects the charging outlet and device's built-in adapter to be part of adapters
pub fn count_jolt_differences(adapters: &Vec<usize>) -> [usize; 3] {
    let (_, counts) =
        adapters
            .iter()
            .skip(1)
            .fold((0, [0, 0, 0]), |(prev, mut counts), adapter| {
                counts[adapter - prev - 1] += 1;

                (*adapter, counts)
            });

    counts
}

// Add the charging outlet and device's built-in adapter to data and sorts it
pub fn prepare(data: &mut Vec<usize>) {
    data.push(0);
    data.push(*data.iter().max().unwrap() + 3);
    data.sort();
}

#[cfg(test)]
mod tests {
    fn create_small_factory() -> Vec<usize> {
        let mut data = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        super::prepare(&mut data);

        data
    }

    fn create_big_factory() -> Vec<usize> {
        let mut data = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        super::prepare(&mut data);

        data
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
