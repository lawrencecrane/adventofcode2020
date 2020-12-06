use std::iter::Iterator;

pub fn to_groups<I>(raw: I) -> Vec<Vec<String>>
where
    I: Iterator<Item = String>,
{
    let (mut groups, last) = raw.fold(
        (Vec::new(), Vec::new()),
        |(mut groups, mut current_group), line| match line.is_empty() {
            true => {
                groups.push(current_group);

                (groups, Vec::new())
            }
            false => {
                current_group.push(line);

                (groups, current_group)
            }
        },
    );

    groups.push(last);

    groups
}

#[cfg(test)]
mod tests {
    fn create_factory() -> Vec<String> {
        vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_to_groups() {
        let groups = create_factory();

        assert_eq!(super::to_groups(groups.into_iter()).len(), 5);
    }
}
