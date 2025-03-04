use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Copy, Clone, Debug)]
struct Mask {
    to_set: u64,
    to_clean: u64,
    to_float: u64,
}

impl Mask {
    pub fn new(bits: &str) -> Self {
        let mut to_set = 0u64;
        let mut to_clean = 0u64;
        let mut to_float = 0u64;
        bits.bytes().rev().enumerate().for_each(|(idx, b)| match b {
            b'0' => to_clean |= 1 << idx,
            b'1' => to_set |= 1 << idx,
            b'X' => to_float |= 1 << idx,
            _ => unreachable!(),
        });

        Mask {
            to_set,
            to_clean,
            to_float,
        }
    }

    pub fn apply(&self, value: u64) -> u64 {
        (value | self.to_set) & !self.to_clean
    }

    pub fn x_powers(&self) -> impl Iterator<Item = Mask> {
        let mut new_mask = *self;
        let to_float = self.to_float;
        new_mask.to_clean = to_float;

        (0..64u64)
            .filter(move |&idx| (to_float & (1 << idx)) != 0)
            .powerset()
            .map(move |floats_to_set| {
                let mut curr_mask = new_mask;
                for idx in floats_to_set {
                    curr_mask.to_set |= 1 << idx;
                    curr_mask.to_clean &= !(1 << idx);
                }
                curr_mask
            })
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    SetMaskInst(Mask),
    WriteMemInst(u64, u64),
}

impl Instruction {
    pub fn parse(line: &str) -> Self {
        peg::parser! {
            pub(crate) grammar parser() for str {
                pub rule instruction() -> Instruction
                = v:(set_mask() / write_mem()) whitespaces() { v }

                rule set_mask() -> Instruction
                = "mask = " bits:$(['X' | '0' | '1']+) {
                    let mask = Mask::new(bits);
                    Instruction::SetMaskInst(mask)
                }

                rule write_mem() -> Instruction
                = "mem[" addr:number() "] = " value:number() {
                    Instruction::WriteMemInst(addr, value)
                }

                rule number() -> u64
                = e:$(['0'..='9']+) { e.parse().unwrap() }

                rule whitespaces()
                = [' ' | '\t' | '\n']*
            }
        }

        parser::instruction(line).unwrap()
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let insts = input.lines().map(Instruction::parse).collect::<Vec<_>>();
    let mut mask;
    let mut mems = HashMap::new();
    let mut addr_masks = vec![];
    for inst in insts {
        match inst {
            Instruction::SetMaskInst(new_mask) => {
                mask = new_mask;
                addr_masks = mask.x_powers().collect();
            }
            Instruction::WriteMemInst(addr, value) => {
                addr_masks.iter().for_each(|amask| {
                    mems.insert(amask.apply(addr), value);
                });
            }
        }
    }
    Some(mems.values().sum())
}

pub fn part_one(input: &str) -> Option<u64> {
    let insts = input.lines().map(Instruction::parse).collect::<Vec<_>>();
    let mut mask = Mask {
        to_set: 0,
        to_clean: 0,
        to_float: 0,
    };
    let mut mems = HashMap::new();

    for inst in insts {
        match inst {
            Instruction::SetMaskInst(new_mask) => mask = new_mask,
            Instruction::WriteMemInst(addr, value) => {
                mems.insert(addr, mask.apply(value));
            }
        }
    }
    Some(mems.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
