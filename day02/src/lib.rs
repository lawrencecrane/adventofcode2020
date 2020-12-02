pub fn n_valid_passwords(passwords: &Vec<&str>) -> usize {
    passwords.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_valid_passwords_test() {
        let passwords = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

        assert_eq!(super::n_valid_passwords(&passwords), 2)
    }
}