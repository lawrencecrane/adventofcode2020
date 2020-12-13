pub fn travel(instructions: &Vec<Instruction>) -> (isize, isize) {
    let (position, _) = instructions.iter().fold(
        ((0, 0), Direction::East),
        |(position, direction), instruction| {
            1 + 1;

            (position, direction)
        },
    );

    position
}

pub fn to_instructions(x: &Vec<String>) -> Vec<Instruction> {
    x.iter().filter_map(Instruction::from_string).collect()
}

impl Instruction {
    fn from_string(x: &String) -> Option<Self> {
        let mut chars = x.chars();

        match (
            Action::from_char(chars.next().unwrap_or(' ')),
            chars.collect::<String>().parse::<usize>(),
        ) {
            (Some(action), Ok(value)) => Some(Instruction { action, value }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    action: Action,
    value: usize,
}

impl Action {
    fn from_char(x: char) -> Option<Self> {
        match x {
            'N' => Some(Action::Base(Direction::North)),
            'S' => Some(Action::Base(Direction::South)),
            'E' => Some(Action::Base(Direction::East)),
            'W' => Some(Action::Base(Direction::West)),
            'L' => Some(Action::Left),
            'R' => Some(Action::Right),
            'F' => Some(Action::Forward),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Base(Direction),
    Left,
    Right,
    Forward,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    fn create_factory() -> Vec<super::Instruction> {
        super::to_instructions(
            &vec!["F10", "N3", "F7", "R90", "F11"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
    }

    #[test]
    fn test_travel() {
        assert_eq!(super::travel(&create_factory()), (17, 8));
    }
}
