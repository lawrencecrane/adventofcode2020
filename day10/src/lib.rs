// Expects the charging outlet and device's built-in adapter to be part of adapters
pub fn n_arrangements(adapters: &Vec<usize>) -> usize {
    let mut i = 0;
    let mut arrangements = find_arrangements(adapters, &vec![]).unwrap();

    while i < arrangements.len() {
        if let Some(mut xs) = find_arrangements(adapters, &arrangements[i]) {
            arrangements.append(&mut xs);
        }

        i += 1;
    }

    arrangements.len() + 1
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

fn find_arrangements(arrangement: &Vec<usize>, included: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let mut found = Vec::new();
    let mut arrangements = Vec::new();

    while let Some(index) = find_arrangement(
        arrangement,
        *found.last().unwrap_or(included.last().unwrap_or(&0)),
        &included,
    ) {
        let mut indeces = included.clone();
        indeces.push(index);

        arrangements.push(indeces);
        found.push(index);
    }

    match found.len() {
        0 => None,
        _ => Some(arrangements),
    }
}

fn find_arrangement(arrangement: &Vec<usize>, from: usize, ignored: &Vec<usize>) -> Option<usize> {
    arrangement
        .iter()
        .enumerate()
        .skip(2 + from)
        .find_map(|(i, adapter)| {
            let mut prevs = (0..i).rev().filter(|x| !ignored.contains(x));

            match (prevs.next(), prevs.next()) {
                (Some(prev), Some(prevprev)) => match *adapter - arrangement[prevprev] <= 3 {
                    true => Some(prev),
                    false => None,
                },
                _ => None,
            }
        })
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

    #[test]
    fn test_find_arrangement() {
        let data = vec![0, 1, 2, 3, 4, 7];

        assert_eq!(super::find_arrangement(&data, 0, &vec![]), Some(1));
        assert_eq!(super::find_arrangement(&data, 1, &vec![1]), Some(2));
        assert_eq!(super::find_arrangement(&data, 2, &vec![1, 2]), None);
    }

    #[test]
    fn test_find_arrangements() {
        let data = vec![0, 1, 2, 3, 4, 7];

        assert_eq!(
            super::find_arrangements(&data, &vec![]),
            Some(vec![vec![1], vec![2], vec![3]])
        );

        assert_eq!(
            super::find_arrangements(&data, &vec![1]),
            Some(vec![vec![1, 2], vec![1, 3]])
        );

        assert_eq!(super::find_arrangements(&data, &vec![1, 2]), None);
    }
}
