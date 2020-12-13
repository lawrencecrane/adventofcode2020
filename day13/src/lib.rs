use std::iter::Iterator;

pub fn find_earliest(timetable: &Timetable) -> (usize, usize) {
    (0, 0)
}

impl Timetable {
    pub fn parse(data: Vec<String>) -> Self {
        Self {
            earliest_depart_time: data[0].parse().unwrap(),
            bus_ids: data[1]
                .split(',')
                .filter(|x| x != &"x")
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Timetable {
    earliest_depart_time: usize,
    bus_ids: Vec<usize>,
}

#[cfg(test)]
mod tests {
    fn create_factory() -> super::Timetable {
        super::Timetable::parse(
            vec!["939", "7,13,x,x,59,x,31,19"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
    }

    #[test]
    fn test_() {
        let data = create_factory();

        assert_eq!(super::find_earliest(&data), (944, 59));
    }
}
