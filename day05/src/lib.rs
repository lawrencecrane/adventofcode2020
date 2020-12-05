pub fn get_seat_id(seat: (usize, usize)) -> usize {
    seat.0 * 8 + seat.1
}

pub fn identify_seat(seat: &String) -> (usize, usize) {
    let row_id = search_partition_space(seat.chars().take(7), (0, 127));
    let column_id = search_partition_space(seat.chars().skip(7).take(3), (0, 7));

    (row_id, column_id)
}

fn search_partition_space<I>(xs: I, space: (usize, usize)) -> usize
where
    I: Iterator<Item = char>,
{
    let (id, _) = xs.fold(space, |space, x| {
        halve_partition_space(Partition::from_char(&x).unwrap(), space)
    });

    id
}

fn halve_partition_space(x: Partition, space: (usize, usize)) -> (usize, usize) {
    let middle = (space.0 + space.1) / 2;

    match x {
        Partition::LOWER => (space.0, middle),
        Partition::UPPER => (middle + 1, space.1),
    }
}

enum Partition {
    LOWER,
    UPPER,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsePartitionError {}

impl Partition {
    fn from_char(input: &char) -> Result<Partition, ParsePartitionError> {
        match input {
            'F' => Ok(Partition::LOWER),
            'B' => Ok(Partition::UPPER),
            'L' => Ok(Partition::LOWER),
            'R' => Ok(Partition::UPPER),
            _ => Err(ParsePartitionError {}),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_identify_seat() {
        assert_eq!(super::identify_seat(&"BFFFBBFRRR".to_string()), (70, 7));
        assert_eq!(super::identify_seat(&"FFFBBBFRRR".to_string()), (14, 7));
        assert_eq!(super::identify_seat(&"BBFFBBFRLL".to_string()), (102, 4));
    }
}
