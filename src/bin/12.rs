use anyhow::anyhow;
use std::str::FromStr;
advent_of_code::solution!(12);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_byte(b: u8) -> anyhow::Result<Self> {
        match b {
            b'N' => Ok(Self::North),
            b'S' => Ok(Self::South),
            b'E' => Ok(Self::East),
            b'W' => Ok(Self::West),
            _ => Err(anyhow!("Must be 'N', 'S', 'E', 'W'")),
        }
    }

    fn next(&mut self) -> &mut Self {
        *self = match *self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        };
        self
    }

    fn rotate(&mut self, degree: i32) {
        let count = ((degree + 360) % 360) / 90;
        (0..count).fold(self, |s, _| s.next());
    }

    fn reverse(&mut self) {
        *self = match *self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum MovingInstruction {
    Moving(Direction, i32),
    Rotate(i32),
    Forward(i32),
}

impl FromStr for MovingInstruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        match bytes[0] {
            dir @ (b'N' | b'S' | b'W' | b'E') => {
                let d = Direction::from_byte(dir)?;
                let count = s[1..].parse::<i32>()?;
                Ok(Self::Moving(d, count))
            }
            dir @ (b'L' | b'R') => {
                let mut count = s[1..].parse::<i32>()?;
                if dir == b'L' {
                    count = -count;
                }
                Ok(Self::Rotate(count))
            }
            b'F' => {
                let count = s[1..].parse::<i32>()?;
                Ok(Self::Forward(count))
            }
            _ => Err(anyhow!("Cannot parse {s} to MovingInstruction")),
        }
    }
}

impl MovingInstruction {
    pub fn rotate(&mut self, degree: i32) {
        match self {
            MovingInstruction::Moving(dir, _) => {
                dir.rotate(degree);
            }
            _ => unreachable!(),
        }
    }

    pub fn mov(&mut self, dir: Direction, count: i32) {
        match self {
            MovingInstruction::Moving(d, c) => match d {
                Direction::North | Direction::South => {
                    if !matches!(dir, Direction::North | Direction::South) {
                        return;
                    }
                    if *d == dir {
                        *c += count;
                    } else {
                        *c -= count;
                        if *c < 0 {
                            d.reverse();
                            *c = c.abs();
                        }
                    }
                }
                Direction::East | Direction::West => {
                    if !matches!(dir, Direction::East | Direction::West) {
                        return;
                    }
                    if *d == dir {
                        *c += count;
                    } else {
                        *c -= count;
                        if *c < 0 {
                            d.reverse();
                            *c = c.abs();
                        }
                    }
                }
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct SpaceShip {
    cordinate: (i32, i32),
    facing: Direction,
    waypoint: [MovingInstruction; 2],
}

impl SpaceShip {
    pub fn new() -> Self {
        Self {
            cordinate: (0, 0),
            facing: Direction::East,
            waypoint: [
                MovingInstruction::Moving(Direction::East, 10),
                MovingInstruction::Moving(Direction::North, 1),
            ],
        }
    }

    fn next(&mut self, inst: MovingInstruction) -> &mut Self {
        match inst {
            MovingInstruction::Rotate(degree) => {
                self.facing.rotate(degree);
            }
            MovingInstruction::Moving(dir, count) => match dir {
                Direction::North => self.cordinate.1 += count,
                Direction::East => self.cordinate.0 += count,
                Direction::South => self.cordinate.1 -= count,
                Direction::West => self.cordinate.0 -= count,
            },
            MovingInstruction::Forward(count) => match self.facing {
                Direction::North => self.cordinate.1 += count,
                Direction::East => self.cordinate.0 += count,
                Direction::South => self.cordinate.1 -= count,
                Direction::West => self.cordinate.0 -= count,
            },
        }
        self
    }

    fn next2(&mut self, inst: MovingInstruction) -> &mut Self {
        match inst {
            MovingInstruction::Rotate(degree) => {
                self.waypoint.iter_mut().for_each(|m| m.rotate(degree));
            }

            MovingInstruction::Moving(dir, count) => {
                self.waypoint.iter_mut().for_each(|m| m.mov(dir, count));
            }

            MovingInstruction::Forward(count) => {
                self.waypoint.iter_mut().for_each(|m| match m {
                    MovingInstruction::Moving(Direction::North, c) => {
                        self.cordinate.1 += *c * count
                    }
                    MovingInstruction::Moving(Direction::South, c) => {
                        self.cordinate.1 -= *c * count
                    }
                    MovingInstruction::Moving(Direction::East, c) => self.cordinate.0 += *c * count,
                    MovingInstruction::Moving(Direction::West, c) => self.cordinate.0 -= *c * count,
                    _ => unreachable!(),
                });
            }
        }
        self
    }

    fn mahaton(&self) -> u64 {
        self.cordinate.0.unsigned_abs() as u64 + self.cordinate.1.unsigned_abs() as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = input
        .lines()
        .flat_map(|line| line.parse::<MovingInstruction>())
        .collect::<Vec<_>>();
    let mut ship = SpaceShip::new();

    instructions
        .into_iter()
        .fold(&mut ship, |ship, inst| ship.next(inst));

    Some(ship.mahaton())
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = input
        .lines()
        .flat_map(|line| line.parse::<MovingInstruction>())
        .collect::<Vec<_>>();
    let mut ship = SpaceShip::new();

    instructions
        .into_iter()
        .fold(&mut ship, |ship, inst| ship.next2(inst));

    Some(ship.mahaton())
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
