fn identify_seat(seat: &String) -> (usize, usize) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_identify_seat() {
        assert_eq!(super::identify_seat(&"BFFFBBFRRR".to_string()), (70, 7));
        assert_eq!(super::identify_seat(&"FFFBBBFRRR".to_string()), (14, 7));
        assert_eq!(super::identify_seat(&"BBFFBBFRLL".to_string()), (102, 4));
    }
}
