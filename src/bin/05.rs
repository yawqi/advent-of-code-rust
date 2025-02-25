advent_of_code::solution!(5);

// * `BFFFBBFRRR`: row `70`, column `7`, seat ID `567`.
// * `FFFBBBFRRR`: row `14`, column `7`, seat ID `119`.
// * `BBFFBBFRLL`: row `102`, column `4`, seat ID `820`.

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| line.split_at(7))
        .map(|(row, col)| {
            let r = row.bytes().fold(0, |cur, mark| match mark {
                b'B' => (cur << 1) + 1,
                _ => cur << 1,
            });
            let c = col.bytes().fold(0, |cur, mark| match mark {
                b'R' => (cur << 1) + 1,
                _ => cur << 1,
            });
            r * 8 + c
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut v = input
        .lines()
        .map(|line| line.split_at(7))
        .map(|(row, col)| {
            let r = row.bytes().fold(0, |cur, mark| match mark {
                b'B' => (cur << 1) + 1,
                _ => cur << 1,
            });
            let c = col.bytes().fold(0, |cur, mark| match mark {
                b'R' => (cur << 1) + 1,
                _ => cur << 1,
            });
            r * 8 + c
        })
        .collect::<Vec<_>>();
    v.sort();
    if let Some(nums) = v.windows(2).find(|v| v[0] + 2 == v[1]) {
        Some(nums[0] + 1)
    } else {
        None
    }
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
