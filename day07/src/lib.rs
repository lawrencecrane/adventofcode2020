use std::collections::HashMap;

pub fn n_bag_contains(bags: &Vec<Bag>, name: String) -> usize {
    let subset = find_children(bags, &name);

    *calculate_values_of_bags(&subset).get(&name).unwrap()
}

fn calculate_values_of_bags(bags: &Vec<Bag>) -> HashMap<String, usize> {
    let mut values = HashMap::new();

    while values.len() < bags.len() {
        bags.iter().for_each(|bag| {
            match calculate_value_of_bag(&values, bag) {
                Some(value) => {
                    values.insert(bag.name.clone(), value);
                }
                None => (),
            };
        });
    }

    values
}

fn calculate_value_of_bag(values: &HashMap<String, usize>, bag: &Bag) -> Option<usize> {
    match bag.can_contain.iter().all(|x| values.contains_key(&x.name)) {
        true => Some(
            bag.can_contain
                .iter()
                .map(|x| x.n + values.get(&x.name).unwrap() * x.n)
                .sum(),
        ),
        false => None,
    }
}

pub fn n_bags_containing(bags: &Vec<Bag>, name: String) -> usize {
    find_parents(bags, &name).iter().count() - 1
}

fn find_children(bags: &Vec<Bag>, name: &String) -> Vec<Bag> {
    search(bags, &name, |bag, parent| parent.contains(&bag.name))
}

fn find_parents(bags: &Vec<Bag>, name: &String) -> Vec<Bag> {
    search(bags, &name, |bag, child| bag.contains(&child.name))
}

fn search<F>(bags: &Vec<Bag>, name: &String, f: F) -> Vec<Bag>
where
    F: Fn(&Bag, &Bag) -> bool,
{
    _search(
        bags,
        vec![bags.iter().find(|x| x.name == *name).unwrap().clone()],
        f,
        0,
    )
}

fn _search<F>(bags: &Vec<Bag>, mut found: Vec<Bag>, f: F, index: usize) -> Vec<Bag>
where
    F: Fn(&Bag, &Bag) -> bool,
{
    let mut new_results: Vec<Bag> = bags
        .iter()
        .filter(|bag| found.iter().skip(index).any(|result| f(bag, result)))
        .filter(|bag| !found.contains(bag))
        .map(|bag| bag.clone())
        .collect();

    match new_results.len() {
        0 => found,
        _ => {
            let new_index = found.len();

            found.append(&mut new_results);

            _search(bags, found, f, new_index)
        }
    }
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

    let can_contain: Vec<Container> = s
        .next()
        .unwrap()
        .split(',')
        .filter(|x| !x.contains("no other bags"))
        .map(|x| {
            let (n, name) =
                split_once(x.split("bag").next().unwrap().trim().to_string(), " ").unwrap();

            Container {
                name,
                n: n.parse().unwrap(),
            }
        })
        .collect();

    Bag { name, can_contain }
}

fn split_once(x: String, pattern: &str) -> Option<(String, String)> {
    let mut s = x.splitn(2, pattern);

    match (s.next(), s.next()) {
        (Some(fst), Some(snd)) => Some((fst.to_string(), snd.to_string())),
        _ => None,
    }
}

impl Bag {
    pub fn contains(&self, name: &String) -> bool {
        self.can_contain.iter().any(|x| x.name == *name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bag {
    name: String,
    can_contain: Vec<Container>,
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
    fn test_n_bag_contains() {
        let bags = super::to_bags(create_factory().into_iter());

        assert_eq!(super::n_bag_contains(&bags, "shiny gold".to_string()), 32);
    }

    #[test]
    fn test_n_bags_containing() {
        let bags = super::to_bags(create_factory().into_iter());

        assert_eq!(super::n_bags_containing(&bags, "shiny gold".to_string()), 4);
    }

    #[test]
    fn test_to_bag() {
        assert_eq!(
            super::to_bag(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string()
            ),
            super::Bag {
                name: "light red".to_string(),
                can_contain: vec![
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
                can_contain: vec![]
            }
        );
    }
}
