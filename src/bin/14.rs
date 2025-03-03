use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy)]
struct Mask<'a>(&'a str);

impl<'a> std::ops::BitAnd<u64> for Mask<'a> {
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

    fn final_sum(&self) -> u64 {
        dbg!(&self.memory).values().sum()
    }
}

use regex::Regex;
pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();
    let p = ('0'..='9').collect_vec();
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
            writes.push(dbg!((v.next().unwrap(), v.next().unwrap())));
        }

        emulator.emulate(mask, writes);
    }

    Some(emulator.final_sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
