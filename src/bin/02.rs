use anyhow::{self, Ok};
use std::{cmp::Ordering, str::FromStr};
advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl RPSChoice {
    fn get_points(&self) -> u64 {
        match self {
            RPSChoice::Rock => 1,
            RPSChoice::Paper => 2,
            RPSChoice::Scissors => 3,
        }
    }

    fn decrypt_enemy(&self, order: Ordering) -> Self {
        match order {
            Ordering::Equal => *self,
            Ordering::Less => *self + (-1),
            Ordering::Greater => *self + 1,
        }
    }
}

impl std::ops::Add<i32> for RPSChoice {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        match self {
            RPSChoice::Rock => {
                if rhs == 0 {
                    RPSChoice::Rock
                } else if rhs > 0 {
                    RPSChoice::Paper
                } else {
                    RPSChoice::Scissors
                }
            }
            RPSChoice::Paper => {
                if rhs == 0 {
                    RPSChoice::Paper
                } else if rhs > 0 {
                    RPSChoice::Scissors
                } else {
                    RPSChoice::Rock
                }
            }
            RPSChoice::Scissors => {
                if rhs == 0 {
                    RPSChoice::Scissors
                } else if rhs > 0 {
                    RPSChoice::Rock
                } else {
                    RPSChoice::Paper
                }
            }
        }
    }
}

impl FromStr for RPSChoice {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPSChoice::Rock),
            "B" | "Y" => Ok(RPSChoice::Paper),
            "C" | "Z" => Ok(RPSChoice::Scissors),
            _ => Err(anyhow::anyhow!("parse failed with {s}")),
        }
    }
}

impl PartialEq for RPSChoice {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RPSChoice::Rock, RPSChoice::Rock)
            | (RPSChoice::Paper, RPSChoice::Paper)
            | (RPSChoice::Scissors, RPSChoice::Scissors) => true,
            _ => false,
        }
    }
}

impl Eq for RPSChoice {}

impl PartialOrd for RPSChoice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }

        match (self, other) {
            (RPSChoice::Rock, RPSChoice::Scissors)
            | (RPSChoice::Scissors, RPSChoice::Paper)
            | (RPSChoice::Paper, RPSChoice::Rock) => Some(Ordering::Greater),
            _ => Some(Ordering::Less),
        }
    }
}

impl Ord for RPSChoice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct RPSGame {
    enemy: RPSChoice,
    me: RPSChoice,
}

impl RPSGame {
    pub fn new(enemy: RPSChoice, me: RPSChoice) -> Self {
        Self { enemy, me }
    }

    pub fn get_points(&self) -> u64 {
        self.me.get_points()
            + match self.me.cmp(&self.enemy) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            }
    }
}

impl FromStr for RPSGame {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(' ')
            .take(2)
            .map(|v| v.parse::<RPSChoice>().unwrap())
            .collect::<Vec<_>>();

        Ok(RPSGame {
            enemy: v[0],
            me: v[1],
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|s| s.parse::<RPSGame>().unwrap())
            .fold(0, |sum, game| sum + game.get_points()),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let v = line.split(" ").collect::<Vec<_>>();
                let enemy = v[0].parse::<RPSChoice>().unwrap();
                let order = match v[1] {
                    "X" => Ordering::Less,
                    "Y" => Ordering::Equal,
                    "Z" => Ordering::Greater,
                    _ => unreachable!(),
                };

                let me = enemy.decrypt_enemy(order);
                RPSGame { enemy, me }
            })
            .fold(0, |sum, game| sum + game.get_points()),
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
