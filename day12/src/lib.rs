use num_complex::Complex;

pub fn travel(instructions: &Vec<Instruction>) -> (isize, isize) {
    let i = Complex::new(0, 1);

    let (position, _) = instructions
        .iter()
        .fold(((0, 0), Complex::new(1, 0)), |(pos, dir), x| {
            match x.action {
                Action::Left => (pos, dir * i),
                Action::Right => (pos, dir * (-1 * i)),
                Action::Forward => ((pos.0 + dir.re * x.value, pos.1 - dir.im * x.value), dir),
                action => {
                    let d = to_rotation(action);

                    ((pos.0 + d.re * x.value, pos.1 - d.im * x.value), dir)
                }
            }
        });

    position
}

fn to_rotation(action: Action) -> Complex<isize> {
    match action {
        Action::North => Complex::new(0, 1),
        Action::South => Complex::new(0, -1),
        Action::East => Complex::new(1, 0),
        Action::West => Complex::new(-1, 0),
        _ => Complex::new(0, 0),
    }
}

pub fn to_instructions(x: &Vec<String>) -> Vec<Instruction> {
    x.iter().filter_map(Instruction::from_string).collect()
}

impl Instruction {
    fn from_string(x: &String) -> Option<Self> {
        let mut chars = x.chars();

        match (
            Action::from_char(chars.next().unwrap_or(' ')),
            chars.collect::<String>().parse::<isize>(),
        ) {
            (Some(action), Ok(value)) => Some(Instruction { action, value }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    action: Action,
    value: isize,
}

impl Action {
    fn from_char(x: char) -> Option<Self> {
        match x {
            'N' => Some(Action::North),
            'S' => Some(Action::South),
            'E' => Some(Action::East),
            'W' => Some(Action::West),
            'L' => Some(Action::Left),
            'R' => Some(Action::Right),
            'F' => Some(Action::Forward),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
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
