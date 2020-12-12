pub fn simulate(layout: Layout) -> Layout {
    layout
}

pub fn noccupied(layout: &Layout) -> usize {
    0
}

pub fn to_layout(data: &Vec<String>) -> Layout {
    data.iter()
        .map(|row| {
            row.chars()
                .map(|c| Position::from_char(c).unwrap())
                .collect()
        })
        .collect()
}

type Layout = Vec<Vec<Position>>;

impl Position {
    fn from_char(x: char) -> Option<Position> {
        match x {
            'L' => Some(Position::EMPTY),
            '#' => Some(Position::OCCUPIED),
            '.' => Some(Position::FLOOR),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Position {
    EMPTY,
    OCCUPIED,
    FLOOR,
}

#[cfg(test)]
mod tests {
    fn create_factory() -> super::Layout {
        super::to_layout(
            &vec![
                "L.LL.LL.LL",
                "LLLLLLL.LL",
                "L.L.L..L..",
                "LLLL.LL.LL",
                "L.LL.LL.LL",
                "L.LLLLL.LL",
                "..L.L.....",
                "LLLLLLLLLL",
                "L.LLLLLL.L",
                "L.LLLLL.LL",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        )
    }

    #[test]
    fn test_simulate() {
        let layout = create_factory();

        assert_eq!(super::noccupied(&super::simulate(layout)), 37);
    }
}
