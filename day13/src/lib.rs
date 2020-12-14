use aoc_utils::range::Range;

pub fn find_earliest_matching_departures(schedules: &Vec<Schedule>) -> usize {
    let max = schedules.iter().max_by(|a, b| a.id.cmp(&b.id)).unwrap();

    Range { step: max.id }
        .into_iter()
        .filter(|x| x > &0)
        .map(|x| x - max.offset)
        .filter(|x| {
            schedules
                .iter()
                .all(|schedule| (x + schedule.offset) % schedule.id == 0)
        })
        .take(1)
        .collect::<Vec<usize>>()[0]
}

pub fn find_earliest(timetable: &Timetable) -> (usize, usize) {
    timetable
        .schedules
        .iter()
        .map(|x| (x.id, x.id * (1 + (timetable.earliest_depart_time / x.id))))
        .min_by(|a, b| (a.1).cmp(&b.1))
        .unwrap()
}

impl Timetable {
    pub fn parse(data: &Vec<String>) -> Self {
        let schedules = data[1]
            .split(',')
            .enumerate()
            .filter(|(_, c)| c != &"x")
            .map(|(offset, id)| Schedule {
                id: id.parse().unwrap(),
                offset,
            })
            .collect();

        Self {
            earliest_depart_time: data[0].parse().unwrap(),
            schedules,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Timetable {
    pub earliest_depart_time: usize,
    pub schedules: Vec<Schedule>,
}

#[derive(Debug, Clone)]
pub struct Schedule {
    id: usize,
    offset: usize,
}

#[cfg(test)]
mod tests {
    fn create_factory() -> super::Timetable {
        super::Timetable::parse(
            &vec!["939", "7,13,x,x,59,x,31,19"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
    }

    fn create_factory_from(ids: &str) -> Vec<super::Schedule> {
        super::Timetable::parse(&vec!["0", ids].iter().map(|s| s.to_string()).collect()).schedules
    }

    #[test]
    fn test_find_earliest() {
        assert_eq!(super::find_earliest(&create_factory()), (59, 944));
    }

    #[test]
    fn test_find_earliest_matching_departures_1() {
        assert_eq!(
            super::find_earliest_matching_departures(&create_factory().schedules),
            1068781
        );
    }

    #[test]
    fn test_find_earliest_matching_departures_2() {
        assert_eq!(
            super::find_earliest_matching_departures(&create_factory_from("17,x,13,19")),
            3417
        );
    }

    #[test]
    fn test_find_earliest_matching_departures_3() {
        assert_eq!(
            super::find_earliest_matching_departures(&create_factory_from("67,7,59,61")),
            754018
        );
    }

    #[test]
    fn test_find_earliest_matching_departures_4() {
        assert_eq!(
            super::find_earliest_matching_departures(&create_factory_from("67,x,7,59,61")),
            779210
        );
    }

    #[test]
    fn test_find_earliest_matching_departures_5() {
        assert_eq!(
            super::find_earliest_matching_departures(&create_factory_from("67,7,x,59,61")),
            1261476
        );
    }

    #[test]
    fn test_find_earliest_matching_departures_6() {
        assert_eq!(
            super::find_earliest_matching_departures(&create_factory_from("1789,37,47,1889")),
            1202161486
        );
    }
}
