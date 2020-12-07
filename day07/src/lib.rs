pub fn n_bags_that_can_contain(bags: &Vec<String>, bag: String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    fn create_factory() -> Vec<String> {
        vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_n_bags_that_can_contain() {
        let bags = create_factory();

        assert_eq!(
            super::n_bags_that_can_contain(&bags, "shiny gold".to_string()),
            4
        );
    }
}
