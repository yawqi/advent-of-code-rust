advent_of_code::solution!(6);

// abc
//
// a
// b
// c
//
// ab
// ac
//
// a
// a
// a
// a
//
// b
//
use std::collections::HashSet;
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .split("\n\n")
            .map(|group| {
                let mut set = HashSet::new();
                group.chars().for_each(|c| match c {
                    c @ 'a'..='z' => {
                        set.insert(c);
                    }
                    _ => {}
                });
                set.len()
            })
            .fold(0, |sum, c| sum + c as u64),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .split("\n\n")
            .map(|group| {
                let mut counts = vec![0; 26];
                let mut len = 0;
                group.lines().for_each(|line| {
                    len += 1;
                    for b in line.as_bytes() {
                        counts[*b as usize - b'a' as usize] += 1;
                    }
                });
                counts.into_iter().filter(|v| *v == len).count()
            })
            .fold(0, |sum, c| sum + c as u64),
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
