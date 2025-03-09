advent_of_code::solution!(12);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Height(u8);
impl Height {
    pub fn new(byte: u8) -> Self {
        Self { 0: byte }
    }

    pub fn can_climb(&self, other: &Height) -> bool {
        self.0 + 1u8 >= other.0
    }
}

use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Mountain {
    roads: Vec<Vec<Height>>,
    start_point: (usize, usize),
    all_start_points: Vec<(usize, usize)>,
    end_point: (usize, usize),
}

impl Mountain {
    pub fn new(mut roads: Vec<Vec<Height>>) -> Self {
        let start_point = roads
            .iter()
            .enumerate()
            .find_map(|(row, heights)| {
                if let Some((col, _)) = heights.iter().enumerate().find(|(col, h)| h.0 == b'S') {
                    Some((row, col))
                } else {
                    None
                }
            })
            .unwrap();
        let end_point = roads
            .iter()
            .enumerate()
            .find_map(|(row, heights)| {
                if let Some((col, _)) = heights.iter().enumerate().find(|(col, h)| h.0 == b'E') {
                    Some((row, col))
                } else {
                    None
                }
            })
            .unwrap();

        roads[start_point.0][start_point.1] = Height::new(b'a');
        roads[end_point.0][end_point.1] = Height::new(b'z');

        let all_start_points = roads
            .iter()
            .enumerate()
            .filter_map(|(row, heights)| {
                if let Some((col, _)) = heights.iter().enumerate().find(|(col, h)| h.0 == b'a') {
                    Some((row, col))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Self {
            roads,
            start_point,
            end_point,
            all_start_points,
        }
    }

    fn can_move_to(&mut self, current: (usize, usize), next: (usize, usize)) -> bool {
        self.roads[current.0][current.1].can_climb(&self.roads[next.0][next.1])
    }

    fn climb(&mut self, start_point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut queues = VecDeque::new();
        let mut paths = VecDeque::new();
        let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut visited = HashSet::new();
        let rlen = self.roads.len();
        let clen = self.roads[0].len();
        paths.push_back(vec![]);

        queues.push_back(start_point);
        loop {
            let len = queues.len();
            for _ in 0..len {
                let mut current_path = paths.pop_front().unwrap();
                let current_pos = queues.pop_front().unwrap();
                if visited.contains(&current_pos) {
                    continue;
                }

                visited.insert(current_pos);
                current_path.push(current_pos);

                if current_pos == self.end_point {
                    return current_path;
                }

                dirs.iter()
                    .filter_map(|(rdiff, cdiff)| {
                        let next_r = *rdiff + current_pos.0 as i64;
                        let next_c = *cdiff + current_pos.1 as i64;
                        if next_r < 0
                            || next_r >= rlen as i64
                            || next_c < 0
                            || next_c >= clen as i64
                            || visited.contains(&(next_r as usize, next_c as usize))
                        {
                            return None;
                        }

                        let next_r = next_r as usize;
                        let next_c = next_c as usize;
                        if self.roads[current_pos.0][current_pos.1]
                            .can_climb(&self.roads[next_r][next_c])
                        {
                            Some((next_r, next_c))
                        } else {
                            None
                        }
                    })
                    .for_each(|next_pos| {
                        queues.push_back(next_pos);
                        paths.push_back(current_path.clone());
                    });
            }
        }
    }

    pub fn find_shortest_path(&mut self) -> Option<u64> {
        let path = self.climb(self.start_point);
        Some(path.len() as u64 - 1)
    }

    pub fn find_shortest_paths(&mut self) -> Option<u64> {
        Some(
            self.all_start_points
                .clone()
                .into_iter()
                .map(|start_point| self.climb(start_point).len())
                .min()
                .unwrap() as u64
                - 1,
        )
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let roads = input
        .lines()
        .map(|line| line.bytes().map(Height::new).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut mountain = Mountain::new(roads);

    mountain.find_shortest_path()
}

pub fn part_two(input: &str) -> Option<u64> {
    let roads = input
        .lines()
        .map(|line| line.bytes().map(Height::new).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut mountain = Mountain::new(roads);

    mountain.find_shortest_paths()
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
