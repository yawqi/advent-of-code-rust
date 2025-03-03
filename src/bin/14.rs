use std::collections::HashMap;

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy)]
struct Mask<'a>(&'a str);

impl std::ops::BitAnd<u64> for Mask<'_> {
    type Output = u64;
    fn bitand(self, rhs: u64) -> Self::Output {
        self.0
            .bytes()
            .rev()
            .enumerate()
            .fold(rhs, |rhs, (idx, byte)| match byte {
                b'X' => rhs,
                b'0' => rhs & !(1u64 << idx),
                b'1' => rhs | (1u64 << idx),
                _ => unreachable!(),
            })
    }
}

impl Mask<'_> {
    pub fn memory_address_and(&self, address: u64) -> impl Iterator<Item = u64> {
        let mut floating_bits = vec![];
        let mut base_value = address;
        self.0
            .bytes()
            .rev()
            .enumerate()
            .for_each(|(idx, c)| match c {
                b'X' => {
                    floating_bits.push(idx);
                    base_value &= !(1 << idx);
                }
                b'1' => base_value |= 1 << idx,
                b'0' => {}
                _ => unreachable!(),
            });

        let permutation_count = 2u64.pow(floating_bits.len() as u32);
        (0..permutation_count).map(move |nth| {
            (0..floating_bits.len())
                .filter(|&bitn| (nth & (1 << bitn)) != 0)
                .fold(base_value, |curr, index_to_bit_to_set| {
                    let bitn = floating_bits[index_to_bit_to_set];
                    curr | (1 << bitn)
                })
        })
    }
}

struct Emulator {
    memory: HashMap<u64, u64>,
}

impl Emulator {
    fn emulate(&mut self, mask: Mask, writes: Vec<(u64, u64)>) {
        writes.into_iter().for_each(|(idx, value)| {
            let new_value = mask & value;
            self.memory.insert(idx, new_value);
        });
    }

    fn emulate_2(&mut self, mask: Mask, writes: Vec<(u64, u64)>) {
        writes
            .into_iter()
            .flat_map(|(addr, value)| mask.memory_address_and(addr).zip(std::iter::repeat(value)))
            .for_each(|(addr, value)| {
                self.memory.insert(addr, value);
            });
    }

    fn final_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

use regex::Regex;
pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();
    let mut emulator = Emulator {
        memory: HashMap::new(),
    };

    let re = Regex::new(r"[\d]+").unwrap();
    while let Some(mask) = lines.next() {
        let mask = Mask(mask.strip_prefix("mask = ").unwrap());
        let mut writes = vec![];

        loop {
            let peek = lines.peek();
            if peek.is_none() || peek.unwrap().starts_with("mask") {
                break;
            }
            let write_op = lines.next().unwrap();
            let mut v = re
                .captures_iter(write_op)
                .map(|v| v.extract::<0>().0.parse::<u64>().unwrap());
            writes.push((v.next().unwrap(), v.next().unwrap()));
        }

        emulator.emulate(mask, writes);
    }

    Some(emulator.final_sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();
    let mut emulator = Emulator {
        memory: HashMap::new(),
    };

    let re = Regex::new(r"[\d]+").unwrap();
    while let Some(mask) = lines.next() {
        let mask = Mask(mask.strip_prefix("mask = ").unwrap());
        let mut writes = vec![];

        loop {
            let peek = lines.peek();
            if peek.is_none() || peek.unwrap().starts_with("mask") {
                break;
            }
            let write_op = lines.next().unwrap();
            let mut v = re
                .captures_iter(write_op)
                .map(|v| v.extract::<0>().0.parse::<u64>().unwrap());
            writes.push((v.next().unwrap(), v.next().unwrap()));
        }

        emulator.emulate_2(mask, writes);
    }

    Some(emulator.final_sum())
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
