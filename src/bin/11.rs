use itertools::Itertools;

advent_of_code::solution!(11);

// #.LL.L#.##
// #LLLLLL.L#
// L.L.L..L..
// #LLL.LL.L#
// #.LL.LL.LL
// #.LLLL#.##
// ..L.L.....
// #LLLLLLLL#
// #.LLLLLL.L
// #.#LLLL.##

fn surrounding_occupied_seats(pos: (usize, usize), map: &[Vec<u8>]) -> usize {
    let dirs = [
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];

    dirs.into_iter()
        .filter_map(|(r, c)| {
            let next_r = r + pos.0 as i64;
            let next_c = c + pos.1 as i64;
            if next_r >= 0
                && next_r < map.len() as i64
                && next_c >= 0
                && next_c < map[0].len() as i64
            {
                Some((next_r as usize, next_c as usize))
            } else {
                None
            }
        })
        .fold(0, |count, pos| {
            if map[pos.0][pos.1] == b'#' {
                count + 1
            } else {
                count
            }
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut prev_map = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    let mut map = prev_map.clone();

    let mut muted = true;
    let rlen = map.len();
    let clen = map[0].len();

    while muted {
        muted = false;
        (0..rlen)
            .flat_map(|r| std::iter::repeat(r).zip(0..clen))
            .for_each(
                |(r, c)| match surrounding_occupied_seats((r, c), &prev_map) {
                    0 if map[r][c] == b'L' => {
                        muted = true;
                        map[r][c] = b'#';
                    }
                    4.. if map[r][c] == b'#' => {
                        muted = true;
                        map[r][c] = b'L';
                    }
                    _ => {}
                },
            );
        prev_map = map.clone();
    }

    Some(map.into_iter().flat_map(|v| v.into_iter()).fold(
        0,
        |sum, c| {
            if c == b'#' {
                sum + 1
            } else {
                sum
            }
        },
    ))
}

fn occupied_seats(pos: (usize, usize), map: &[Vec<u8>]) -> usize {
    let dirs = [
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];

    dirs.into_iter()
        .filter_map(|(r, c)| {
            let mut next_r = r + pos.0 as i64;
            let mut next_c = c + pos.1 as i64;
            while next_r >= 0
                && next_r < map.len() as i64
                && next_c >= 0
                && next_c < map[0].len() as i64
            {
                if map[next_r as usize][next_c as usize] != b'.' {
                    return Some((next_r as usize, next_c as usize));
                }
                next_r += r;
                next_c += c;
            }
            None
        })
        .fold(0, |count, pos| {
            if map[pos.0][pos.1] == b'#' {
                count + 1
            } else {
                count
            }
        })
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut prev_map = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    let mut map = prev_map.clone();

    let mut muted = true;
    let rlen = map.len();
    let clen = map[0].len();

    while muted {
        muted = false;
        (0..rlen)
            .flat_map(|r| std::iter::repeat(r).zip(0..clen))
            .for_each(|(r, c)| match occupied_seats((r, c), &prev_map) {
                0 if map[r][c] == b'L' => {
                    muted = true;
                    map[r][c] = b'#';
                }
                5.. if map[r][c] == b'#' => {
                    muted = true;
                    map[r][c] = b'L';
                }
                _ => {}
            });
        prev_map = map.clone();
    }

    Some(map.into_iter().flat_map(|v| v.into_iter()).fold(
        0,
        |sum, c| {
            if c == b'#' {
                sum + 1
            } else {
                sum
            }
        },
    ))
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
