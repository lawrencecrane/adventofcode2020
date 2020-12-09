pub fn find_first_mismatch(numbers: Vec<usize>, preamble: usize) -> usize {
    0
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

        assert_eq!(super::find_first_mismatch(numbers, 5), 127);
    }
}
