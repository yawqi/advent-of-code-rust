use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn get_priority(byte: u8) -> u64 {
    match byte {
        b'a'..=b'z' => (byte - b'a') as u64 + 1,
        b'A'..=b'Z' => (byte - b'A') as u64 + 27,
        _ => unreachable!(),
    }
}
struct RuckSack<'a> {
    part1: &'a [u8],
    part2: &'a [u8],
    common: u8,
}

impl<'a> RuckSack<'a> {
    pub fn new(line: &'a str) -> Self {
        let bytes = line.as_bytes();
        let (part1, part2) = bytes.split_at(bytes.len() / 2);
        let set: HashSet<&u8> = part1.iter().collect();
        let common = *part2.iter().find(|c| set.contains(*c)).unwrap();

        Self {
            part1,
            part2,
            common,
        }
    }

    pub fn priority(&self) -> u64 {
        get_priority(self.common)
    }

    pub fn get_set(&'a self) -> HashSet<u8> {
        self.part1
            .iter()
            .copied()
            .chain(self.part2.iter().copied())
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(RuckSack::new)
            .map(|sack| sack.priority())
            .sum(),
    )
}

use itertools::Itertools;
pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|group| {
                let set = group
                    .into_iter()
                    .map(RuckSack::new)
                    .fold(HashSet::new(), |set, sack| {
                        if set.is_empty() {
                            sack.get_set().clone()
                        } else {
                            set.intersection(&sack.get_set()).copied().collect()
                        }
                    });
                assert!(set.len() == 1);
                get_priority(set.into_iter().next().unwrap())
            })
            .sum(),
    )
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
