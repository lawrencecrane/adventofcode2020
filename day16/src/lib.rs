use std::collections::HashMap;

pub fn calculate_error_rate(data: &TicketData) -> usize {
    data.nearby
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|field| !is_valid(*field, &data.rules))
                .sum::<usize>()
        })
        .sum()
}

fn is_valid(field: &usize, rules: &Rules) -> bool {
    rules
        .values()
        .any(|((a1, a2), (b1, b2))| (field >= a1 && field <= a2) || (field >= b1 && field <= b2))
}

pub fn to_ticket_data(raw: &Vec<String>) -> TicketData {
    let is_not_empty = |x: &&String| x != &&"".to_string();

    let rules: Rules = raw
        .iter()
        .take_while(is_not_empty)
        .filter_map(parse_rule)
        .collect();

    let my = to_tickets(
        raw.iter()
            .skip_while(|x| !x.contains("your ticket"))
            .skip(1)
            .take_while(is_not_empty),
    )
    .pop()
    .unwrap();

    let nearby = to_tickets(
        raw.iter()
            .skip_while(|x| !x.contains("nearby tickets"))
            .skip(1),
    );

    TicketData { rules, my, nearby }
}

fn parse_rule(raw: &String) -> Option<Rule> {
    let mut s = raw.split(": ");

    match (
        s.next(),
        s.next().map(|v| {
            let mut s = v.split(" or ");
            match (s.next(), s.next()) {
                (Some(a), Some(b)) => Some((to_range(a), to_range(b))),
                _ => None,
            }
        }),
    ) {
        (Some(key), Some(Some((Some(a), Some(b))))) => Some((key.to_string(), (a, b))),
        _ => None,
    }
}

fn to_range(x: &str) -> Option<(usize, usize)> {
    let mut s = x.split('-');

    match (s.next(), s.next()) {
        (Some(a), Some(b)) => Some((a.parse().unwrap(), b.parse().unwrap())),
        _ => None,
    }
}

fn to_tickets<'a, I>(raw: I) -> Vec<Vec<usize>>
where
    I: Iterator<Item = &'a String>,
{
    raw.map(parse_ticket).collect()
}

fn parse_ticket(raw: &String) -> Vec<usize> {
    raw.split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[derive(Debug)]
pub struct TicketData {
    rules: Rules,
    my: Vec<usize>,
    nearby: Vec<Vec<usize>>,
}

type Rules = HashMap<String, ((usize, usize), (usize, usize))>;

type Rule = (String, ((usize, usize), (usize, usize)));

#[cfg(test)]
mod tests {
    use super::{calculate_error_rate, to_ticket_data, TicketData};

    fn create_factory() -> TicketData {
        to_ticket_data(
            &vec![
                "class: 1-3 or 5-7",
                "row: 6-11 or 33-44",
                "seat: 13-40 or 45-50",
                "",
                "your ticket:",
                "7,1,14",
                "",
                "nearby tickets:",
                "7,3,47",
                "40,4,50",
                "55,2,20",
                "38,6,12",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        )
    }

    #[test]
    fn test_calculate_error_rate() {
        assert_eq!(calculate_error_rate(&create_factory()), 71);
    }
}
