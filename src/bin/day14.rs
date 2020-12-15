use itertools::sorted;
use std::collections::HashMap;
use std::fs::read_to_string;

fn number_to_bool_array(n: u64) -> [bool; 36] {
    let mut n = n;
    let mut result = [false; 36];

    for i in (0..36).rev() {
        result[i] = (n % 2) != 0;
        n = n / 2;
    }

    return result;
}

fn bool_array_to_number(array: &[bool; 36]) -> u64 {
    let mut result = 0;
    let mut power = 0;

    for i in (0..36).rev() {
        result += match array[i] {
            true => 1,
            false => 0,
        } * 2_u64.pow(power);
        power += 1;
    }

    result
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum MaskValue {
    X,
    Zero,
    One,
}

impl MaskValue {
    fn from_char(c: &char) -> MaskValue {
        match c {
            'X' => MaskValue::X,
            '1' => MaskValue::One,
            '0' => MaskValue::Zero,
            _ => panic!("unknown character {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BitMask([MaskValue; 36]);

impl BitMask {
    fn from_string(text: &str) -> BitMask {
        let mut result: [MaskValue; 36] = [MaskValue::X; 36];
        for (i, c) in text.chars().enumerate() {
            result[i] = MaskValue::from_char(&c);
        }

        BitMask(result)
    }

    fn apply_1(&self, value: &[bool; 36]) -> [bool; 36] {
        let mut result: [bool; 36] = [false; 36];

        for i in 0..value.len() {
            result[i] = match self.0[i] {
                MaskValue::X => value[i],
                MaskValue::One => true,
                MaskValue::Zero => false,
            }
        }
        result
    }

    fn apply_2(&self, value: &[bool; 36]) -> Vec<u64> {
        let mut result: Vec<u64> = vec![0];
        let mut power = 0;

        for i in (0..value.len()).rev() {
            match self.0[i] {
                MaskValue::X => {
                    result = result
                        .iter()
                        .flat_map(|x| vec![x + 2_u64.pow(power), *x])
                        .collect()
                }
                MaskValue::One => result.iter_mut().for_each(|x| *x = *x + 2_u64.pow(power)),
                MaskValue::Zero => result
                    .iter_mut()
                    .for_each(|x| *x = *x + 2_u64.pow(power) * value[i] as u64),
            }
            power += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::MaskValue::*;
    use super::*;

    #[test]
    fn test_mask_from_string() {
        assert_eq!(
            BitMask::from_string("X100110110X011000101000101XX11001X11"),
            BitMask([
                X, One, Zero, Zero, One, One, Zero, One, One, Zero, X, Zero, One, One, Zero, Zero,
                Zero, One, Zero, One, Zero, Zero, Zero, One, Zero, One, X, X, One, One, Zero, Zero,
                One, X, One, One
            ])
        );
    }

    #[test]
    fn test_mask_apply() {
        assert_eq!(
            BitMask::from_string("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply_1(&[
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, true, false, true, true
            ]),
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, true, false, false, true, false, false, true
            ]
        );
    }

    #[test]
    fn test_number_to_bool_array() {
        assert_eq!(bool_array_to_number(&number_to_bool_array(12)), 12);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("mem[5201] = 1838761"),
            Line::MemorySet((5201, 1838761))
        );
        assert_eq!(
            parse_line("mask = X100110110X011000101000101XX11001X11"),
            Line::Mask(BitMask::from_string("X100110110X011000101000101XX11001X11"))
        );
    }

    #[test]
    fn test_apply_2() {
        assert_eq!(
            sorted(
                BitMask::from_string("000000000000000000000000000000X1001X").apply_2(&[
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, true, false, true,
                    false, true, false
                ])
            )
            .collect::<Vec<_>>(),
            sorted(vec!(26, 27, 58, 59)).collect::<Vec<_>>()
        )
    }
}

#[derive(Debug)]
struct ProgramState {
    current_mask: BitMask,
    memory: HashMap<u64, u64>,
}

#[derive(Debug, PartialEq)]
enum Line {
    Mask(BitMask),
    MemorySet((u64, u64)),
}

fn parse_line(line: &str) -> Line {
    if line.starts_with("mem") {
        let index: u64 = line[4..line.find(']').unwrap()].parse().unwrap();
        let value: u64 = line.split(" = ").nth(1).unwrap().parse().unwrap();
        return Line::MemorySet((index, value));
    } else {
        return Line::Mask(BitMask::from_string(line.split(" = ").nth(1).unwrap()));
    }
}

fn main() {
    let content = read_to_string("./inputs/day14.txt").expect("file not found");

    let mut program_state = ProgramState {
        current_mask: BitMask::from_string("XXXX"),
        memory: HashMap::new(),
    };

    // Task 1:
    for line in content.lines() {
        match parse_line(line) {
            Line::Mask(mask) => {
                program_state.current_mask = mask.clone();
            }
            Line::MemorySet((address, value)) => {
                let masked_value = program_state
                    .current_mask
                    .apply_1(&number_to_bool_array(value));
                program_state
                    .memory
                    .insert(address, bool_array_to_number(&masked_value));
            }
        }
    }

    // Result: 6559449933360
    println!(
        "{}",
        program_state
            .memory
            .iter()
            .map(|(_, v)| *v)
            .filter(|v| *v != 0)
            .sum::<u64>()
    );

    // Task 2:
    let mut program_state = ProgramState {
        current_mask: BitMask::from_string("XXXX"),
        memory: HashMap::new(),
    };
    for line in content.lines() {
        match parse_line(line) {
            Line::Mask(mask) => {
                program_state.current_mask = mask.clone();
            }
            Line::MemorySet((address, value)) => {
                let masked_addresses = program_state
                    .current_mask
                    .apply_2(&number_to_bool_array(address));
                for masked_address in masked_addresses {
                    program_state.memory.insert(masked_address, value);
                }
            }
        }
    }
    // Result: 3369767240513
    println!(
        "{}",
        program_state
            .memory
            .iter()
            .map(|(_, v)| *v)
            .filter(|v| *v != 0)
            .sum::<u64>()
    );
}
