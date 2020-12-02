use std::iter::Iterator;

pub fn n_valid_passwords_by_count_policy(passwords: &Vec<Password>) -> usize {
    passwords.iter().filter(|pw| check_count_policy(pw)).count()
}

pub fn check_count_policy(password: &Password) -> bool {
    let count = password
        .value
        .chars()
        .filter(|c| c == &password.policy.value)
        .count();

    count >= password.policy.min && count <= password.policy.max
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

    match (s.next(), s.next()) {
        (Some(policy), Some(password)) => match to_policy(policy) {
            Some(policy) => Some(Password {
                policy,
                value: String::from(password),
            }),
            _ => None,
        },
        _ => None,
    }
}

pub fn to_policy(policy: &str) -> Option<Policy> {
    let mut s = policy.split_whitespace();

    match (s.next(), s.next()) {
        (Some(minmax), Some(letter)) => {
            let mut ss = minmax.split('-');

            match (ss.next(), ss.next()) {
                (Some(min), Some(max)) => Some(Policy {
                    min: min.parse().unwrap(),
                    max: max.parse().unwrap(),
                    value: letter.chars().nth(0).unwrap(),
                }),
                _ => None,
            }
        }
        _ => None,
    }
}

pub struct Password {
    pub policy: Policy,
    pub value: String,
}

pub struct Policy {
    pub min: usize,
    pub max: usize,
    pub value: char,
}

#[cfg(test)]
mod tests {
    #[test]
    fn n_valid_passwords_by_count_policy_test() {
        let passwords = super::to_passwords(
            vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
                .iter()
                .map(|s| s.to_string()),
        );

        assert_eq!(super::n_valid_passwords_by_count_policy(&passwords), 2)
    }
}
