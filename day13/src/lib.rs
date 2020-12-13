use std::iter::Iterator;

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
        let (schedules, _) =
            data[1]
                .split(',')
                .fold((Vec::new(), 0), |(mut schedules, offset), id| {
                    match id.parse::<usize>() {
                        Ok(id) => {
                            schedules.push(Schedule { id, offset });
                            (schedules, 0)
                        }
                        _ => (schedules, offset + 1),
                    }
                });

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

    #[test]
    fn test_find_earliest() {
        assert_eq!(super::find_earliest(&create_factory()), (59, 944));
    }
}
