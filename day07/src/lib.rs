pub fn n_bags_that_can_contain(bags: &Vec<Bag>, bag: String) -> usize {
    0
}

pub fn to_bags<I>(bags: I) -> Vec<Bag>
where
    I: Iterator<Item = String>,
{
    bags.map(&to_bag).collect()
}

fn to_bag(bag: String) -> Bag {
    let mut s = bag.split("contain");

    let name = s
        .next()
        .unwrap()
        .split("bag")
        .next()
        .unwrap()
        .trim()
        .to_string();

    let contains: Vec<Container> = s
        .next()
        .unwrap()
        .split(',')
        .map(|x| {
            let (n, name) =
                split_once(x.split("bag").next().unwrap().trim().to_string(), " ").unwrap();

            Container {
                name,
                n: n.parse().unwrap(),
            }
        })
        .collect();

    Bag { name, contains }
}

fn split_once(x: String, pattern: &str) -> Option<(String, String)> {
    let mut s = x.splitn(2, pattern);

    match (s.next(), s.next()) {
        (Some(fst), Some(snd)) => Some((fst.to_string(), snd.to_string())),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bag {
    name: String,
    contains: Vec<Container>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Container {
    name: String,
    n: usize,
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
        let bags = super::to_bags(create_factory().into_iter());

        assert_eq!(
            super::n_bags_that_can_contain(&bags, "shiny gold".to_string()),
            4
        );
    }

    #[test]
    fn test_to_bag() {
        assert_eq!(
            super::to_bag(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string()
            ),
            super::Bag {
                name: "light red".to_string(),
                contains: vec![
                    super::Container {
                        name: "bright white".to_string(),
                        n: 1
                    },
                    super::Container {
                        name: "muted yellow".to_string(),
                        n: 2,
                    }
                ]
            }
        );
    }

    #[test]
    fn test_to_bag_no_other_bags() {
        assert_eq!(
            super::to_bag("dim tomato bags contain no other bags.".to_string()),
            super::Bag {
                name: "dim tomato".to_string(),
                contains: vec![]
            }
        );
    }
}
