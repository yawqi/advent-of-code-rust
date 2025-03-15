advent_of_code::solution!(17);

use anyhow::{anyhow, Result};
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::LazyLock,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl std::ops::Add for Coordinate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<(i64, i64)> for Coordinate {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Coordinate {
    pub fn index_into<'a, T, U>(&self, v: &'a [T]) -> &'a U
    where
        T: AsRef<[U]>,
    {
        &(v[self.x as usize].as_ref())[self.y as usize]
    }

    pub fn index_into_mut<'a, T, U>(&self, v: &'a mut [T]) -> &'a mut U
    where
        T: AsMut<[U]>,
    {
        &mut (v[self.x as usize].as_mut())[self.y as usize]
    }

    pub fn in_legal_range(&self, col_count: i64) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < col_count
    }
}

#[derive(Debug, Clone)]
struct Rock {
    height: u8,
    blocks: Vec<Vec<bool>>,
    coordinate: Coordinate,
}

impl Rock {
    fn get_all_coords(&self) -> impl Iterator<Item = Coordinate> + '_ {
        let corner = self.coordinate;
        self.blocks
            .iter()
            .enumerate()
            .flat_map(move |(x_index, col)| {
                col.iter().enumerate().filter_map(move |(y_index, ele)| {
                    if *ele {
                        Some(corner + Coordinate::from((x_index as i64, y_index as i64)))
                    } else {
                        None
                    }
                })
            })
    }

    pub fn try_move<T>(&mut self, mov: Move, map: &[T]) -> Result<()>
    where
        T: AsRef<[bool]> + std::fmt::Debug,
    {
        if self
            .get_all_coords()
            .map(|cord| cord + mov.into())
            .all(|cord| cord.in_legal_range(map.len() as i64) && !*cord.index_into(map))
        {
            self.coordinate += mov.into();
            return Ok(());
        }
        Err(anyhow!("not in valid range"))
    }

    pub fn set_map<T>(&mut self, map: &mut [T])
    where
        T: AsMut<[bool]>,
    {
        self.get_all_coords().for_each(|cord| {
            *cord.index_into_mut(map) = true;
        });
    }

    pub fn set_cord(&mut self, cord: Coordinate) {
        self.coordinate = cord;
    }

    pub fn get_highest(&self) -> u64 {
        self.coordinate.y as u64 + self.height as u64
    }
}

static LINE: LazyLock<Rock> = LazyLock::new(|| Rock {
    height: 1,
    blocks: vec![vec![true]; 4],
    coordinate: (0, 0).into(),
});

static PLUS: LazyLock<Rock> = LazyLock::new(|| Rock {
    height: 3,
    blocks: vec![
        vec![false, true, false],
        vec![true; 3],
        vec![false, true, false],
    ],
    coordinate: (0, 0).into(),
});

static MIRROR_L: LazyLock<Rock> = LazyLock::new(|| Rock {
    height: 3,
    blocks: vec![vec![true], vec![true], vec![true; 3]],
    coordinate: (0, 0).into(),
});

static VLINE: LazyLock<Rock> = LazyLock::new(|| Rock {
    height: 4,
    blocks: vec![vec![true; 4]],
    coordinate: (0, 0).into(),
});

static SQUARE: LazyLock<Rock> = LazyLock::new(|| Rock {
    height: 2,
    blocks: vec![vec![true; 2]; 2],
    coordinate: (0, 0).into(),
});

#[derive(Debug, Clone)]
struct Tower<const N: usize> {
    highest: u64,
    occupied: [Vec<bool>; N],
    states: HashMap<(u64, usize, usize), (usize, u64)>,
    pattern_found: bool,
}

impl<const N: usize> Tower<N> {
    pub fn new() -> Self {
        Self {
            highest: 0,
            occupied: [(); N].map(|_| Vec::new()),
            states: HashMap::new(),
            pattern_found: false,
        }
    }

    pub fn drop_one_rock(
        &mut self,
        mut rock: Rock,
        moves: &mut impl Iterator<Item = (usize, Move)>,
    ) -> usize {
        rock.set_cord((2, self.highest as i64 + 3).into());
        for ele in self.occupied.iter_mut() {
            ele.extend(vec![false; 3 + rock.height as usize].into_iter());
        }

        for (mov_index, mov) in moves {
            let _ = rock.try_move(mov, &self.occupied);
            if rock.try_move(Move::Down, &self.occupied).is_err() {
                rock.set_map(&mut self.occupied);
                self.highest = self.highest.max(rock.get_highest());
                return mov_index;
            }
        }

        unreachable!()
    }

    pub fn check_pattern_cycle(
        &mut self,
        rock_index: usize,
        mov_index: usize,
        iter_index: usize,
    ) -> Option<(usize, u64)> {
        if self.pattern_found || self.highest < 8 {
            return None;
        }

        let state = self
            .occupied
            .iter()
            .flat_map(|v| (&v[self.highest as usize - 8..self.highest as usize]).iter())
            .fold(0u64, |state, occupied| {
                if *occupied {
                    state << 1 | 1
                } else {
                    state << 1
                }
            });

        if let Entry::Occupied(e) = self.states.entry((state, rock_index, mov_index)) {
            self.pattern_found = true;
            return Some(e.get().clone());
        } else {
            self.states
                .insert((state, rock_index, mov_index), (iter_index, self.highest));
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
    Down,
}

impl From<Move> for Coordinate {
    fn from(value: Move) -> Self {
        match value {
            Move::Left => Self { x: -1, y: 0 },
            Move::Right => Self { x: 1, y: 0 },
            Move::Down => Self { x: 0, y: -1 },
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    //let mut moves = input
    //    .trim()
    //    .chars()
    //    .map(|c| match c {
    //        '>' => Move::Right,
    //        '<' => Move::Left,
    //        _ => unreachable!(),
    //    })
    //    .enumerate()
    //    .cycle();

    //let rocks = vec![
    //    LINE.clone(),
    //    PLUS.clone(),
    //    MIRROR_L.clone(),
    //    VLINE.clone(),
    //    SQUARE.clone(),
    //]
    //.into_iter()
    //.enumerate()
    //.cycle()
    //.take(2022);

    //let mut tower = Tower::<7>::new();
    //for (rock in rocks {
    //    tower.drop_one_rock(rock, &mut moves);
    //}

    //Some(tower.highest)
    let mut moves = input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Move::Right,
            '<' => Move::Left,
            _ => unreachable!(),
        })
        .enumerate()
        .cycle();

    let mut rocks = vec![
        LINE.clone(),
        PLUS.clone(),
        MIRROR_L.clone(),
        VLINE.clone(),
        SQUARE.clone(),
    ]
    .into_iter()
    .enumerate()
    .cycle();

    let mut tower = Tower::<7>::new();
    let mut i = 0usize;
    let total_round = 2022;
    let mut cycle_height = 0u64;
    let mut cycle_count = 0usize;

    while i < total_round {
        let (rock_index, rock) = rocks.next().unwrap();
        let mov_index = tower.drop_one_rock(rock, &mut moves);
        if let Some((idx, height)) = tower.check_pattern_cycle(rock_index, mov_index, i) {
            let cycle_len = i - idx;
            cycle_count = (total_round - i) / cycle_len;
            i += cycle_count * cycle_len;
            cycle_height = tower.highest - height;
        }
        i += 1;
    }

    Some(tower.highest + cycle_count as u64 * cycle_height)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut moves = input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Move::Right,
            '<' => Move::Left,
            _ => unreachable!(),
        })
        .enumerate()
        .cycle();

    let mut rocks = vec![
        LINE.clone(),
        PLUS.clone(),
        MIRROR_L.clone(),
        VLINE.clone(),
        SQUARE.clone(),
    ]
    .into_iter()
    .enumerate()
    .cycle();

    let mut tower = Tower::<7>::new();
    let mut i = 0usize;
    let total_round = 1000000000000usize;
    let mut cycle_height = 0u64;
    let mut cycle_count = 0usize;

    while i < total_round {
        let (rock_index, rock) = rocks.next().unwrap();
        let mov_index = tower.drop_one_rock(rock, &mut moves);
        if let Some((idx, height)) = tower.check_pattern_cycle(rock_index, mov_index, i) {
            let cycle_len = i - idx;
            cycle_count = (total_round - i) / cycle_len;
            i += cycle_count * cycle_len;
            cycle_height = tower.highest - height;
        }
        i += 1;
    }

    Some(tower.highest + cycle_count as u64 * cycle_height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3068));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
