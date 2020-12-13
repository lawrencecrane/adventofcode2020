use num_complex::Complex;

pub fn travel_with_waypoint(instructions: &Vec<Instruction>) -> (isize, isize) {
    let (position, _) = instructions
        .iter()
        .fold(((0, 0), (10, -1)), |(ship, waypoint), x| match x.action {
            Action::Forward => (
                (ship.0 + x.value * waypoint.0, ship.1 + x.value * waypoint.1),
                waypoint,
            ),
            Action::Left | Action::Right => {
                let rotation = match if x.action == Action::Left {
                    360 - x.value
                } else {
                    x.value
                } {
                    90 => CLOCKWISE_90_DEG,
                    180 => CLOCKWISE_180_DEG,
                    270 => CLOCKWISE_270_DEG,
                    r => panic!("Unexpected rotation, got {} expected: 90, 180, 270", r),
                };

                (ship, rotate(waypoint, rotation))
            }
            action => {
                let dir = to_rotation(action);

                (
                    ship,
                    (waypoint.0 + dir.re * x.value, waypoint.1 - dir.im * x.value),
                )
            }
        });

    position
}

pub fn travel(instructions: &Vec<Instruction>) -> (isize, isize) {
    let i = Complex::new(0, 1);
    let minus_i = Complex::new(0, -1);

    let (position, _) = instructions
        .iter()
        .fold(((0, 0), Complex::new(1, 0)), |(pos, dir), x| {
            match x.action {
                Action::Left => (pos, dir * i.powi((x.value / 90) as i32)),
                Action::Right => (pos, dir * minus_i.powi((x.value / 90) as i32)),
                action => {
                    let d = match action {
                        Action::Forward => dir,
                        _ => to_rotation(action),
                    };

                    ((pos.0 + d.re * x.value, pos.1 - d.im * x.value), dir)
                }
            }
        });

    position
}

// These are actually counter clockwise matrices for the labeled degrees,
// but as the y axis is inverted in our coodrinate system these trasform to clockwise.
const CLOCKWISE_90_DEG: [[isize; 2]; 2] = [[0, -1], [1, 0]];
const CLOCKWISE_180_DEG: [[isize; 2]; 2] = [[-1, 0], [0, -1]];
const CLOCKWISE_270_DEG: [[isize; 2]; 2] = [[0, 1], [-1, 0]];

fn rotate(vector: (isize, isize), mat: [[isize; 2]; 2]) -> (isize, isize) {
    (
        vector.0 * mat[0][0] + vector.1 * mat[0][1],
        vector.0 * mat[1][0] + vector.1 * mat[1][1],
    )
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

    #[test]
    fn test_travel_with_waypoint() {
        assert_eq!(super::travel_with_waypoint(&create_factory()), (214, 72));
    }
}
