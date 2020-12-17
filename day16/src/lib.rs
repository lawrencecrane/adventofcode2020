use std::collections::HashMap;

pub fn to_ticket_data(raw: Vec<String>) -> TicketData {}

struct TicketData {
    rules: HashMap<String, ((usize, usize), (usize, usize))>,
    my: Vec<usize>,
    nearby: Vec<Vec<usize>>,
}

#[cfg(test)]
mod tests {
    fn create_factory() -> Vec<String> {
        vec![
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
        .collect()
    }
}
