use std::collections::HashMap;

pub fn run(program: &Program) -> usize {
    let (memory, _) =
        program
            .iter()
            .fold(
                (HashMap::new(), None),
                |(mut memory, mask), instruction| match instruction {
                    (Some(new_mask), _) => (memory, Some(new_mask)),
                    (_, Some(assignment)) => {
                        memory.insert(
                            assignment.address,
                            apply_mask(mask.unwrap(), assignment.value),
                        );

                        (memory, mask)
                    }
                    _ => (memory, mask),
                },
            );

    memory.values().sum()
}

fn apply_mask(mask: &Mask, n: usize) -> usize {
    mask.iter().fold(n, |value, bit| {
        match (bit.is_set, kth_bit_is_set(n, bit.kth)) {
            (true, false) => value + bit.value,
            (false, true) => value - bit.value,
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
                (Some(a), Some(b)) if a.starts_with("mask") => Some((Some(to_mask(b)), None)),
                (Some(a), Some(b)) if a.starts_with("mem[") => Some((
                    None,
                    Some(Assignment {
                        value: b.parse().unwrap(),
                        address: a.replace("mem[", "").replace("]", "").parse().unwrap(),
                    }),
                )),
                _ => None,
            }
        })
        .collect()
}

fn to_mask(raw: &str) -> Mask {
    raw.chars()
        .rev()
        .enumerate()
        .filter_map(|(i, bit)| match bit {
            'X' => None,
            _ => Some(MaskBit {
                kth: i + 1,
                value: (2 as usize).pow(i as u32),
                is_set: bit == '1',
            }),
        })
        .collect()
}

type Program = Vec<Instruction>;

type Instruction = (Option<Mask>, Option<Assignment>);

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
    is_set: bool,
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

    #[test]
    fn test_run() {
        assert_eq!(super::run(&create_factory()), 165);
    }

    #[test]
    fn test_kth_bit_is_set() {
        assert_eq!(super::kth_bit_is_set(0, 2), false);
        assert_eq!(super::kth_bit_is_set(101, 2), false);
    }

    #[test]
    fn test_apply_mask() {
        let factory = create_factory();
        let (fst, _) = factory.first().unwrap();
        let mask = fst.to_owned().unwrap();

        assert_eq!(super::apply_mask(&mask, 11), 73);
        assert_eq!(super::apply_mask(&mask, 101), 101);
        assert_eq!(super::apply_mask(&mask, 0), 64);
    }
}
