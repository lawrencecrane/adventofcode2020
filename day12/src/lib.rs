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
            'N' => Some(Action::N),
            'S' => Some(Action::S),
            'E' => Some(Action::E),
            'W' => Some(Action::W),
            'L' => Some(Action::L),
            'R' => Some(Action::R),
            'F' => Some(Action::F),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
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
    fn test_() {}
}
