use std::ops::RangeInclusive;

use itertools::Itertools;
use parse_rule::parse_rule;
use peg::parser;
use std::collections::HashSet;

advent_of_code::solution!(15);

//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cord {
    x: i64,
    y: i64,
}

impl Cord {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn clone_col(&self, row: i64) -> Self {
        let mut ret = self.clone();
        ret.y = row;
        ret
    }

    pub fn range_within_distance(&self, dist: i64, row: i64) -> Option<RangeInclusive<i64>> {
        let nearest = self.clone_col(row);
        let nearest_dist = self.distance_to(&nearest);

        if nearest_dist > dist {
            None
        } else {
            Some(self.x - (dist - nearest_dist)..=self.x + (dist - nearest_dist))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    sensor: Cord,
    beacon: Cord,
}

impl Rule {
    pub fn new(sensor: Cord, beacon: Cord) -> Self {
        Self { sensor, beacon }
    }

    pub fn range_at_row(&self, row: i64) -> Option<RangeInclusive<i64>> {
        let dist = self.sensor.distance_to(&self.beacon);
        self.sensor.range_within_distance(dist, row)
    }
}

// Sensor at x=3923513, y=2770279: closest beacon is at x=3866712, y=2438950
parser! {
    pub grammar parse_rule() for str {
        pub rule parse_rule() -> Rule
        = "Sensor at x=" x:numbers() ", y=" y:numbers() ": closest beacon is at x=" xb:numbers() ", y=" yb:numbers()
        { Rule::new(Cord::new(x,y), Cord::new(xb, yb)) }

        rule numbers() -> i64
        = num:$("-"?['0'..='9']+) { num.parse::<i64>().unwrap() }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let rules = input
        .lines()
        .map(|line| parse_rule(line.trim()).unwrap())
        .collect_vec();

    let mut all_range = rules
        .iter()
        .flat_map(|r| r.range_at_row(2000000))
        .flatten()
        .collect::<HashSet<i64>>();

    for rule in rules {
        if rule.beacon.y == 2000000 && all_range.contains(&rule.beacon.x) {
            all_range.remove(&rule.beacon.x);
        }
    }

    Some(all_range.len() as u64)
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
