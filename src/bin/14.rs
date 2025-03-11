use std::fmt::Display;

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Space {
    Empty,
    Wall,
    Sand,
    Void,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Empty => write!(f, " "),
            Self::Sand => write!(f, "O"),
            Self::Wall => write!(f, "#"),
            Self::Void => write!(f, "~"),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    matrix: [[Space; 1000]; 200],
    dropping_cord: (usize, usize),
    total_sand: usize,
    floor: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.matrix {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn new() -> Self {
        Self {
            matrix: [[Space::Empty; 1000]; 200],
            dropping_cord: (501, 1),
            total_sand: 0,
            floor: 3,
        }
    }

    pub fn add_boundary(&mut self, boundary: Vec<(usize, usize)>) {
        for line_cord in boundary.windows(2) {
            let (start_col, start_row) = line_cord[0];
            let (end_col, end_row) = line_cord[1];

            assert!(start_col == end_col || start_row == end_row);
            let max_row = std::cmp::max(start_row, end_row);
            let min_row = std::cmp::min(start_row, end_row);
            let max_col = std::cmp::max(start_col, end_col);
            let min_col = std::cmp::min(start_col, end_col);
            for c in min_col..=max_col {
                self.matrix[start_row][c] = Space::Wall;
            }

            for r in min_row..=max_row {
                self.matrix[r][start_col] = Space::Wall;
            }

            self.floor = self.floor.max(max_row + 2);
        }
    }

    pub fn add_floor(&mut self) {
        self.matrix[self.floor]
            .iter_mut()
            .for_each(|v| *v = Space::Wall);
    }

    pub fn one_more_sand(&mut self) -> bool {
        let mut start_cord = self.dropping_cord;
        if matches!(self.cord_state(&self.dropping_cord), Space::Sand) {
            return false;
        }

        while let Some(next_cord) = self.drop_this_sand(&start_cord) {
            if start_cord == next_cord {
                self.total_sand += 1;
                self.matrix[next_cord.1][next_cord.0] = Space::Sand;
                return true;
            }
            start_cord = next_cord;
        }
        false
    }

    fn cord_state(&self, cord: &(usize, usize)) -> Space {
        if cord.0 >= self.matrix[0].len() - 1
            || cord.0 == 0
            || cord.1 >= self.matrix.len() - 1
            || cord.1 == 0
        {
            return Space::Void;
        }

        self.matrix[cord.1][cord.0]
    }

    fn drop_this_sand(&mut self, cord: &(usize, usize)) -> Option<(usize, usize)> {
        let dirs = [(0, 1), (-1, 1), (1, 1)];
        for dir in dirs {
            let next_cord = (
                (cord.0 as i32 + dir.0) as usize,
                (cord.1 as i32 + dir.1) as usize,
            );
            match self.cord_state(&next_cord) {
                Space::Void => return None,
                Space::Empty => return Some(next_cord),
                Space::Wall | Space::Sand => continue,
            }
        }
        Some(*cord)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let all_cords = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|cord| {
                    let (x, y) = cord.split_once(',').unwrap();
                    (
                        x.parse::<usize>().unwrap() + 1,
                        y.parse::<usize>().unwrap() + 1,
                    )
                })
                .collect_vec()
        })
        .collect_vec();

    let mut map = Map::new();
    for cords in all_cords {
        map.add_boundary(cords);
    }

    while map.one_more_sand() {}
    Some(map.total_sand as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let all_cords = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|cord| {
                    let (x, y) = cord.split_once(',').unwrap();
                    (
                        x.parse::<usize>().unwrap() + 1,
                        y.parse::<usize>().unwrap() + 1,
                    )
                })
                .collect_vec()
        })
        .collect_vec();

    let mut map = Map::new();
    for cords in all_cords {
        map.add_boundary(cords);
    }
    map.add_floor();

    while map.one_more_sand() {}
    Some(map.total_sand as u64)
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
