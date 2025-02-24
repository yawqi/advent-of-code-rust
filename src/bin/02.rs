advent_of_code::solution!(2);
use anyhow::{anyhow, Error, Ok, Result};
use nom::{
    bytes::complete::{tag, take, take_while},
    IResult, Parser,
};
use std::{ops::RangeInclusive, str::FromStr};

fn parse_password_policy(i: &str) -> IResult<&str, PasswordPolicy> {
    let (i, name) = take_while(|c: char| c.is_numeric())(i)?;
    let start = name.parse::<u64>().unwrap();

    let (i, _) = tag("-").parse(i)?;

    let (i, name) = take_while(|c: char| c.is_numeric())(i)?;
    let end = name.parse::<u64>().unwrap();

    let (i, _) = tag(" ").parse(i)?;
    let (i, target) = take(1usize)(i)?;

    let (i, _) = tag(": ")(i)?;
    IResult::Ok((
        i,
        PasswordPolicy {
            range: RangeInclusive::new(start as usize, end as usize),
            target: target.bytes().take(1).next().unwrap(),
        },
    ))
}

struct PasswordPolicy {
    pub range: RangeInclusive<usize>,
    pub target: u8,
}

impl PasswordPolicy {
    pub fn is_valid(&self, password: &str) -> bool {
        self.range
            .contains(&password.bytes().filter(|c| *c == self.target).count())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let count = input
        .lines()
        .map(parse_password_policy)
        .filter_map(Result::ok)
        .filter(|(password, policy)| policy.is_valid(password))
        .count();

    Some(count as u64)
}

struct Password2;

impl FromStr for Password2 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s.split(char::is_whitespace).take(3).collect::<Vec<_>>();
        let (lower, upper) = res[0].split_once('-').ok_or(anyhow!("cannot split"))?;
        let target = res[1].chars().take(1).next().ok_or(anyhow!("nope"))?;
        let lower = lower.parse::<usize>()?;
        let upper = upper.parse::<usize>()?;

        let mut chars = res[2].chars();

        let c1 = chars
            .by_ref()
            .skip(lower - 1)
            .next()
            .ok_or(anyhow!("invalid pos"))?;

        let c2 = chars
            .skip(upper - lower - 1)
            .next()
            .ok_or(anyhow!("invalid pos"))?;

        if (c1 == target && c2 == target) || (c1 != target && c2 != target) {
            Err(anyhow!("invalid occurence count"))
        } else {
            Ok(Password2)
        }
    }
}
pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(str::parse::<Password2>)
            .filter(Result::is_ok)
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
