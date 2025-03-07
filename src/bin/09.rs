use std::collections::HashSet;
use std::str::FromStr;

use anyhow::anyhow;
use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Move {
    dir: Dir,
    step: i64,
}

impl FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, len) = s.split_once(' ').unwrap();
        let step = len.parse::<i64>()?;
        let dir = match dir {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => Err(anyhow!("not existed"))?,
        };
        Ok(Self { dir, step })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn move_one(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.y += 1,
            Dir::Down => self.y -= 1,
            Dir::Right => self.x += 1,
            Dir::Left => self.x -= 1,
        }
    }

    fn is_around(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn move_towards(&mut self, other: &Self) {
        if self.is_around(other) {
            return;
        }

        if (other.x - self.x).abs() > 1 || (other.y != self.y && other.x != self.x) {
            if other.x > self.x {
                self.move_one(Dir::Right);
            } else {
                self.move_one(Dir::Left);
            }
        }

        if (other.y - self.y).abs() > 1 || (other.y != self.y && other.x != self.x) {
            if other.y > self.y {
                self.move_one(Dir::Up);
            } else {
                self.move_one(Dir::Down);
            }
        }
    }
}

struct Simulate {
    head: Position,
    tail: Vec<Position>,
    visited: HashSet<Position>,
}

impl Simulate {
    pub fn new(tail_count: usize) -> Self {
        let head = Position { x: 0, y: 0 };
        let tail = vec![head; tail_count];
        let mut visited = HashSet::new();
        visited.insert(*tail.last().unwrap());
        Simulate {
            head,
            tail,
            visited,
        }
    }

    pub fn execute_one(&mut self, movement: Move) {
        for _ in 0..movement.step {
            self.head.move_one(movement.dir);

            self.tail[0].move_towards(&self.head);
            for idx in 1..self.tail.len() {
                let prev = self.tail[idx - 1];
                self.tail[idx].move_towards(&prev);
            }
            self.visited.insert(*self.tail.last().unwrap());
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut simulator = Simulate::new(1);
    input
        .lines()
        .flat_map(str::parse::<Move>)
        .for_each(|mov| simulator.execute_one(mov));

    Some(simulator.visited.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut simulator = Simulate::new(9);
    input
        .lines()
        .flat_map(str::parse::<Move>)
        .for_each(|mov| simulator.execute_one(mov));

    Some(simulator.visited.len() as u64)
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
