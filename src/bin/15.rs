use std::ops::{RangeBounds, RangeInclusive};

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

fn ranges(rules: &[Rule], row: i64) -> Vec<RangeInclusive<i64>> {
    let mut ranges = rules.iter().flat_map(|r| r.range_at_row(row)).collect_vec();
    ranges.sort_by_key(|r| *r.start());

    ranges
        .into_iter()
        .coalesce(|a, b| {
            if *a.end() + 1 >= *b.start() {
                Ok(*a.start()..=*a.end().max(b.end()))
            } else {
                Err((a, b))
            }
        })
        .collect_vec()
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

    let ranges = ranges(&rules, 2000000);
    let mut all_count = ranges
        .iter()
        .fold(0, |sum, r| sum + r.end() - r.start() + 1);

    let to_be_removes = rules
        .iter()
        .filter_map(|r| {
            if r.beacon.y == 2000000 {
                Some(r.beacon.x)
            } else {
                None
            }
        })
        .collect::<HashSet<i64>>();

    for to_be_remove in to_be_removes {
        for r in &ranges {
            if r.contains(&to_be_remove) {
                all_count -= 1;
            }
        }
    }

    Some(all_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rules = input
        .lines()
        .map(|line| parse_rule(line.trim()).unwrap())
        .collect_vec();

    let x_range = 0..=4000000;
    let y_range = 0..=4000000;

    let point = y_range
        .into_iter()
        .find_map(|y| {
            let ranges = ranges(&rules, y);

            let mut ranges = ranges.into_iter().filter_map(|range| {
                let new_range =
                    *range.start().max(x_range.start())..=*range.end().min(x_range.end());
                if new_range.start() > new_range.end() {
                    None
                } else {
                    Some(new_range)
                }
            });

            ranges.nth(1).map(|r| (*r.start() - 1, y))
        })
        .unwrap();

    dbg!(point);
    Some((point.0 * 4000000 + point.1) as u64)
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
