use std::ops::RangeInclusive;

advent_of_code::solution!(4);

fn parse_range(s: &str) -> RangeInclusive<u64> {
    let (n1, n2) = s.split_once('-').unwrap();
    n1.parse().unwrap()..=n2.parse().unwrap()
}

fn range_contains<T: PartialOrd>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.contains(r2.start()) && r1.contains(r2.end())
}

fn range_overlap<T: PartialOrd>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.contains(r2.start()) || r1.contains(r2.end())
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let (p1, p2) = line.split_once(',').unwrap();
                let r1 = parse_range(p1);
                let r2 = parse_range(p2);
                if range_contains(&r1, &r2) || range_contains(&r2, &r1) {
                    Some(())
                } else {
                    None
                }
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let (p1, p2) = line.split_once(',').unwrap();
                let r1 = parse_range(p1);
                let r2 = parse_range(p2);
                if range_overlap(&r1, &r2) || range_overlap(&r2, &r1) {
                    Some(())
                } else {
                    None
                }
            })
            .count() as u64,
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
