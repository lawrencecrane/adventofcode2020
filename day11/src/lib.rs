use itertools::Itertools;
use std::collections::HashMap;

pub fn simulate(layout: &Layout) -> Layout {
    let adjacent_map: HashMap<(usize, usize), Vec<(usize, usize)>> = layout
        .iter()
        .map(|(k, _)| (*k, adjacent_seats(k, layout)))
        .collect();

    _simulate(layout.clone(), adjacent_map)
}

fn _simulate(
    previous: Layout,
    adjacent_map: HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> Layout {
    let next: Layout = previous
        .iter()
        .map(|(k, seat)| {
            (
                *k,
                next_state(seat, &previous, adjacent_map.get(k).unwrap()),
            )
        })
        .collect();

    match next == previous {
        true => next,
        false => _simulate(next, adjacent_map),
    }
}

fn next_state(seat: &Seat, layout: &Layout, adjacent: &Vec<(usize, usize)>) -> Seat {
    match adjacent
        .iter()
        .filter(|(x, y)| layout.get(&(*x, *y)).unwrap() == &Seat::OCCUPIED)
        .count()
    {
        0 => Seat::OCCUPIED,
        n if n >= 4 => Seat::EMPTY,
        _ => *seat,
    }
}

fn adjacent_seats(point: &(usize, usize), layout: &Layout) -> Vec<(usize, usize)> {
    let (x, y) = (point.0 as isize, point.1 as isize);

    ((x - 1)..(x + 2))
        .cartesian_product((y - 1)..(y + 2))
        .filter(|a| match a {
            p if p == &(x, y) => false,
            p if p.0 < 0 || p.1 < 0 => false,
            p if !layout.contains_key(&(p.0 as usize, p.1 as usize)) => false,
            _ => true,
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

pub fn noccupied(layout: &Layout) -> usize {
    layout.values().filter(|x| x == &&Seat::OCCUPIED).count()
}

pub fn to_layout(data: &Vec<String>) -> Layout {
    data.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.chars()
                .enumerate()
                .map(move |(y, c)| ((x, y), Seat::from_char(c)))
                .filter(|(_, seat)| seat.is_some())
                .map(|(pos, seat)| (pos, seat.unwrap()))
        })
        .collect()
}

type Layout = HashMap<(usize, usize), Seat>;

impl Seat {
    fn from_char(x: char) -> Option<Seat> {
        match x {
            'L' => Some(Seat::EMPTY),
            '#' => Some(Seat::OCCUPIED),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Seat {
    EMPTY,
    OCCUPIED,
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
