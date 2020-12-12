use std::collections::{HashMap, HashSet};

pub fn simulate(layout: &Layout, max_adjacent_distance: isize) -> Layout {
    _simulate(
        layout.clone(),
        adjacent_seats(layout, max_adjacent_distance),
        4,
    )
}

fn _simulate(previous: Layout, adjacent_map: AdjacentMap, nmax_adjacent: usize) -> Layout {
    let next: Layout = previous
        .iter()
        .map(|(k, seat)| {
            (
                *k,
                next_state(seat, &previous, adjacent_map.get(k).unwrap(), nmax_adjacent),
            )
        })
        .collect();

    match next == previous {
        true => next,
        false => _simulate(next, adjacent_map, nmax_adjacent),
    }
}

fn next_state(
    seat: &Seat,
    layout: &Layout,
    adjacent: &HashSet<(isize, isize)>,
    nmax_adjacent: usize,
) -> Seat {
    match adjacent
        .iter()
        .filter(|(x, y)| layout.get(&(*x, *y)).unwrap() == &Seat::OCCUPIED)
        .count()
    {
        0 => Seat::OCCUPIED,
        n if n >= nmax_adjacent => Seat::EMPTY,
        _ => *seat,
    }
}

fn adjacent_seats(layout: &Layout, max_distance: isize) -> AdjacentMap {
    let mut points = layout.keys().collect::<Vec<&(isize, isize)>>();

    points.sort();

    points.iter().fold(HashMap::new(), |mut m, point| {
        let is_valid = |p: &&&(isize, isize)| *p != point && is_in_range(point, p, &max_distance);

        if let Some(right) = points.iter().filter(is_valid).find(|(_, y)| y == &point.1) {
            m.mutually_insert(**point, **right);
        }

        if let Some(bottom) = points.iter().filter(is_valid).find(|(x, _)| x == &point.0) {
            m.mutually_insert(**point, **bottom);
        }

        if let Some(br_diag) = points
            .iter()
            .filter(is_valid)
            .find(|p| is_diagonal(point, p, |xd, yd| yd > 0 && xd > 0))
        {
            m.mutually_insert(**point, **br_diag);
        }

        if let Some(bl_diag) = points
            .iter()
            .filter(is_valid)
            .find(|p| is_diagonal(point, p, |xd, yd| yd > 0 && xd < 0))
        {
            m.mutually_insert(**point, **bl_diag);
        }

        m
    })
}

/// # Arguments
/// * `f` - delta x (b.0 - a.0) and delta y (b.1 - a.1) will be passed as arguments
///         can be used to further check which diagonal the vector is in
fn is_diagonal<F>(a: &(isize, isize), b: &(isize, isize), f: F) -> bool
where
    F: Fn(isize, isize) -> bool,
{
    let yd = b.1 - a.1;
    let xd = b.0 - a.0;

    yd.abs() == xd.abs() && f(xd, yd)
}

fn is_in_range(origin: &(isize, isize), point: &(isize, isize), max_distance: &isize) -> bool {
    (point.0 - origin.0).abs() <= *max_distance && (point.1 - origin.1).abs() <= *max_distance
}

trait MutuallyInsert {
    fn mutually_insert(&mut self, fst: (isize, isize), snd: (isize, isize));
}

impl MutuallyInsert for AdjacentMap {
    fn mutually_insert(&mut self, fst: (isize, isize), snd: (isize, isize)) {
        self.entry(fst).or_insert(HashSet::new()).insert(snd);
        self.entry(snd).or_insert(HashSet::new()).insert(fst);
    }
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
                .map(move |(y, c)| ((x as isize, y as isize), Seat::from_char(c)))
                .filter(|(_, seat)| seat.is_some())
                .map(|(pos, seat)| (pos, seat.unwrap()))
        })
        .collect()
}

type AdjacentMap = HashMap<(isize, isize), HashSet<(isize, isize)>>;

type Layout = HashMap<(isize, isize), Seat>;

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

        assert_eq!(super::noccupied(&super::simulate(&layout, 1)), 37);
    }
}
