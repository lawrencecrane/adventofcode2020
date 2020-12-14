use std::collections::HashMap;

pub fn run<F>(program: &Program, decoder: F) -> usize
where
    F: Fn(&mut HashMap<usize, usize>, &Mask, &Assignment) -> (),
{
    let (memory, _) =
        program
            .iter()
            .fold(
                (HashMap::new(), None),
                |(mut memory, mask), instruction| match instruction {
                    Instruction::Mask(new_mask) => (memory, Some(new_mask)),
                    Instruction::Assignment(assignment) => {
                        decoder(&mut memory, mask.unwrap(), assignment);

                        (memory, mask)
                    }
                },
            );

    memory.values().sum()
}

pub fn decoder_v2(memory: &mut HashMap<usize, usize>, mask: &Mask, assignment: &Assignment) {
    apply_mask_v2(mask, assignment.address)
        .iter()
        .for_each(|address| {
            memory.insert(*address, assignment.value);
        });
}

pub fn decoder_v1(memory: &mut HashMap<usize, usize>, mask: &Mask, assignment: &Assignment) {
    memory.insert(assignment.address, apply_mask_v1(mask, assignment.value));
}

fn apply_mask_v2(mask: &Mask, n: usize) -> Vec<usize> {
    mask.iter().fold(vec![n], |mut values, bit| match bit.flag {
        Flag::Schrodinger => values
            .iter()
            .flat_map(|x| match kth_bit_is_set(*x, bit.kth) {
                true => vec![*x, *x - bit.value],
                false => vec![*x, *x + bit.value],
            })
            .collect(),
        Flag::One => {
            values
                .iter_mut()
                .filter(|x| !kth_bit_is_set(**x, bit.kth))
                .for_each(|x| *x += bit.value);

            values
        }
        _ => values,
    })
}

fn apply_mask_v1(mask: &Mask, n: usize) -> usize {
    mask.iter().fold(n, |value, bit| {
        match (bit.flag, kth_bit_is_set(n, bit.kth)) {
            (Flag::One, false) => value + bit.value,
            (Flag::Zero, true) => value - bit.value,
            _ => value,
        }
    })
}

fn kth_bit_is_set(n: usize, k: usize) -> bool {
    n & (1 << (k - 1)) > 0
}

pub fn to_program(raw: Vec<String>) -> Program {
    raw.iter()
        .filter_map(|x| {
            let mut s = x.split(" = ");

            match (s.next(), s.next()) {
                (Some(a), Some(b)) if a.starts_with("mask") => Some(Instruction::Mask(to_mask(b))),
                (Some(a), Some(b)) if a.starts_with("mem[") => {
                    Some(Instruction::Assignment(Assignment {
                        value: b.parse().unwrap(),
                        address: a.replace("mem[", "").replace("]", "").parse().unwrap(),
                    }))
                }

                _ => None,
            }
        })
        .collect()
}

fn to_mask(raw: &str) -> Mask {
    raw.chars()
        .rev()
        .enumerate()
        .map(|(i, bit)| MaskBit {
            kth: i + 1,
            value: (2 as usize).pow(i as u32),
            flag: match bit {
                '1' => Flag::One,
                '0' => Flag::Zero,
                _ => Flag::Schrodinger,
            },
        })
        .collect()
}

type Program = Vec<Instruction>;

pub enum Instruction {
    Assignment(Assignment),
    Mask(Mask),
}

#[derive(Debug, Clone, Copy)]
pub struct Assignment {
    value: usize,
    address: usize,
}

type Mask = Vec<MaskBit>;

#[derive(Debug, Clone, Copy)]
pub struct MaskBit {
    kth: usize,
    value: usize,
    flag: Flag,
}

#[derive(Debug, Clone, Copy)]
enum Flag {
    One,
    Zero,
    Schrodinger,
}

#[cfg(test)]
mod tests {
    fn create_factory() -> super::Program {
        super::to_program(
            vec![
                "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
                "mem[8] = 11",
                "mem[7] = 101",
                "mem[8] = 0",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        )
    }

    fn create_small_factory() -> super::Program {
        super::to_program(
            vec![
                "mask = 000000000000000000000000000000X1001X",
                "mem[42] = 100",
                "mask = 00000000000000000000000000000000X0XX",
                "mem[26] = 1",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        )
    }

    #[test]
    fn test_run_v1() {
        assert_eq!(super::run(&create_factory(), super::decoder_v1), 165);
    }

    #[test]
    fn test_run_v2() {
        assert_eq!(super::run(&create_small_factory(), super::decoder_v2), 208);
    }
}
