use itertools::Itertools;

pub fn simulate(layout: &Layout) -> Layout {
    let seats = get_seat_indices(&layout);
    let max = *seats.last().unwrap().last().unwrap();

    _simulate(layout.clone(), seats, max)
}

fn _simulate(previous: Layout, seats: Vec<Vec<(usize, usize)>>, max: (usize, usize)) -> Layout {
    let next: Vec<Vec<Position>> = seats
        .iter()
        .map(|row| {
            row.iter()
                .map(|seat| next_state(seat, &previous, &max))
                .collect()
        })
        .collect();

    match next == previous {
        true => next,
        false => _simulate(next, seats, max),
    }
}

fn get_seat_indices(layout: &Layout) -> Vec<Vec<(usize, usize)>> {
    layout
        .iter()
        .enumerate()
        .map(|(x, row)| row.iter().enumerate().map(|(y, _)| (x, y)).collect())
        .collect()
}

fn next_state(seat: &(usize, usize), layout: &Layout, max: &(usize, usize)) -> Position {
    if layout[seat.0][seat.1] == Position::FLOOR {
        return Position::FLOOR;
    }

    let noccupied = adjacent_seats(seat, max)
        .iter()
        .filter(|(x, y)| layout[*x][*y] == Position::OCCUPIED)
        .count();

    match noccupied {
        0 => Position::OCCUPIED,
        x if x >= 4 => Position::EMPTY,
        _ => layout[seat.0][seat.1],
    }
}

fn adjacent_seats(point: &(usize, usize), max: &(usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = (point.0 as isize, point.1 as isize);

    ((x - 1)..(x + 2))
        .cartesian_product((y - 1)..(y + 2))
        .filter(|a| match a {
            p if p == &(x, y) => false,
            p if p.0 < 0 || p.1 < 0 => false,
            p if p.0 > max.0 as isize || p.1 > max.1 as isize => false,
            _ => true,
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

pub fn noccupied(layout: &Layout) -> usize {
    layout
        .iter()
        .map(|row| {
            row.iter()
                .filter(|seat| **seat == Position::OCCUPIED)
                .count()
        })
        .sum()
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

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
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

        assert_eq!(super::noccupied(&super::simulate(&layout)), 37);
    }
}
