advent_of_code::solution!(6);
use std::collections::HashSet;
pub fn part_one(input: &str) -> Option<u64> {
    input
        .as_bytes()
        .windows(4)
        .zip(4..)
        .find(|(w, _)| {
            let s = w.iter().collect::<HashSet<_>>();
            s.len() == 4
        })
        .map(|(_, idx)| idx)
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .as_bytes()
        .windows(14)
        .zip(14..)
        .find(|(w, _)| {
            let s = w.iter().collect::<HashSet<_>>();
            s.len() == 14
        })
        .map(|(_, idx)| idx)
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
