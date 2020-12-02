use std::iter::Iterator;

pub fn n_valid_passwords(passwords: &Vec<Password>) -> usize {
    passwords.len()
}

pub fn to_passwords<I>(passwords: I) -> Vec<Password>
where
    I: Iterator<Item = String>,
{
    passwords
        .map(|s| to_password(s))
        .filter(|pw| pw.is_some())
        .map(|pw| pw.unwrap())
        .collect()
}

pub fn to_password(password: String) -> Option<Password> {
    let mut s = password.split(": ");
    let (policy, password) = (s.next(), s.next());

    match (policy, password) {
        (Some(policy), Some(password)) => Some(Password {
            policy: String::from(policy),
            value: String::from(password),
        }),
        _ => None,
    }
}

pub struct Password {
    pub policy: String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_valid_passwords_test() {
        let passwords = super::to_passwords(
            vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
                .iter()
                .map(|s| s.to_string()),
        );

        assert_eq!(super::n_valid_passwords(&passwords), 2)
    }
}
