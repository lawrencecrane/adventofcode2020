pub fn execute_corrupted_program(code: &Vec<Code>) -> isize {
    let mut index: usize = 0;

    loop {
        index += code
            .iter()
            .skip(index)
            .position(|x| x.instruction == Instruction::NOP || x.instruction == Instruction::JMP)
            .unwrap();

        let (acc, terminated) = _execute(code, 0, 0, Vec::new(), &|x, pos| match pos == index {
            true => swap_corrupted_instruction(x),
            false => x,
        });

        if terminated {
            break acc;
        }

        index += 1;
    }
}

fn swap_corrupted_instruction(code: Code) -> Code {
    match code.instruction {
        Instruction::NOP => Code {
            instruction: Instruction::JMP,
            value: code.value,
        },
        Instruction::JMP => Code {
            instruction: Instruction::NOP,
            value: code.value,
        },
        _ => code,
    }
}

pub fn execute(code: &Vec<Code>) -> (isize, bool) {
    _execute(code, 0, 0, Vec::new(), &|x, _| x)
}

fn _execute<F>(
    code: &Vec<Code>,
    acc: isize,
    index: usize,
    mut executed: Vec<usize>,
    attach: F,
) -> (isize, bool)
where
    F: Fn(Code, usize) -> Code,
{
    match (executed.contains(&index), index == code.len()) {
        (true, _) => (acc, false),
        (_, true) => (acc, true),
        _ => {
            executed.push(index);

            let instruction = attach(code[index], index);

            match instruction.instruction {
                Instruction::NOP => _execute(code, acc, index + 1, executed, attach),
                Instruction::ACC => {
                    _execute(code, acc + instruction.value, index + 1, executed, attach)
                }
                Instruction::JMP => _execute(
                    code,
                    acc,
                    (index as isize + instruction.value) as usize,
                    executed,
                    attach,
                ),
            }
        }
    }
}

pub fn to_codes<I>(raw: I) -> Vec<Code>
where
    I: Iterator<Item = String>,
{
    raw.map(|x| to_code(x).unwrap()).collect()
}

fn to_code(x: String) -> Option<Code> {
    let mut s = x.split_whitespace();

    match (s.next(), s.next()) {
        (Some(instruction), Some(value)) => Some(Code {
            instruction: Instruction::from_string(instruction).unwrap(),
            value: value.parse().unwrap(),
        }),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Code {
    instruction: Instruction,
    value: isize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    NOP,
    ACC,
    JMP,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseInstructionError {}

impl Instruction {
    fn from_string(input: &str) -> Result<Instruction, ParseInstructionError> {
        match input {
            "nop" => Ok(Instruction::NOP),
            "acc" => Ok(Instruction::ACC),
            "jmp" => Ok(Instruction::JMP),
            _ => Err(ParseInstructionError {}),
        }
    }
}

#[cfg(test)]
mod tests {
    fn create_factory() -> Vec<String> {
        vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_execute() {
        let code = super::to_codes(create_factory().into_iter());

        assert_eq!(super::execute(&code), (5, false))
    }

    #[test]
    fn test_execute_corrupted_program() {
        let code = super::to_codes(create_factory().into_iter());

        assert_eq!(super::execute_corrupted_program(&code), 8)
    }
}
