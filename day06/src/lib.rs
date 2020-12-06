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
    #[test]
    fn test() {}
}
