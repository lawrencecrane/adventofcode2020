use itertools::Itertools;
use std::iter::Iterator;

pub fn count_yes_answers(groups: &Vec<Vec<String>>) -> usize {
    groups.iter().map(n_yes_answers).sum()
}

fn n_yes_answers(group: &Vec<String>) -> usize {
    group.iter().flat_map(|x| x.chars()).unique().count()
}

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

    #[test]
    fn test_count_yes_answers() {
        let groups = super::to_groups(create_factory().into_iter());

        assert_eq!(super::count_yes_answers(&groups), 11);
    }
}
