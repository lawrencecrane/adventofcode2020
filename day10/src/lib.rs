pub fn count_jolt_differences(adapters: &Vec<usize>) -> (usize, usize, usize) {
    (0, 0, 0)
}

#[cfg(test)]
mod tests {
    fn create_small_factory() -> Vec<usize> {
        vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
    }

    fn create_big_factory() -> Vec<usize> {
        vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]
    }

    #[test]
    fn test_count_jolt_differences() {
        assert_eq!(
            super::count_jolt_differences(&create_small_factory()),
            (7, 0, 5)
        );

        assert_eq!(
            super::count_jolt_differences(&create_big_factory()),
            (22, 0, 10)
        );
    }
}
