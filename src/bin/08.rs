use itertools::Itertools;

advent_of_code::solution!(8);

pub fn is_visible(forrest: &[Vec<u64>], r: usize, c: usize) -> bool {
    if (0..r).all(|ridx| forrest[ridx][c] < forrest[r][c])
        || (r + 1..forrest.len()).all(|ridx| forrest[ridx][c] < forrest[r][c])
    {
        return true;
    }

    if (0..c).all(|cidx| forrest[r][cidx] < forrest[r][c])
        || (c + 1..forrest[0].len()).all(|cidx| forrest[r][cidx] < forrest[r][c])
    {
        return true;
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let forrest = input
        .lines()
        .map(|row| row.bytes().map(|b| (b - b'0') as u64).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let r = forrest.len();
    let c = forrest[0].len();

    Some(
        (0..r)
            .cartesian_product(0..c)
            .filter(|(row, col)| is_visible(&forrest, *row, *col))
            .count() as u64,
    )
}

pub fn count_distances(forrest: &[Vec<u64>], r: usize, c: usize) -> u64 {
    [
        (0..r)
            .rev()
            .take_while_inclusive(|ridx| forrest[*ridx][c] < forrest[r][c])
            .count() as u64,
        (r + 1..forrest.len())
            .take_while_inclusive(|ridx| forrest[*ridx][c] < forrest[r][c])
            .count() as u64,
        (0..c)
            .rev()
            .take_while_inclusive(|cidx| forrest[r][*cidx] < forrest[r][c])
            .count() as u64,
        (c + 1..forrest[0].len())
            .take_while_inclusive(|cidx| forrest[r][*cidx] < forrest[r][c])
            .count() as u64,
    ]
    .into_iter()
    .product()
}

pub fn part_two(input: &str) -> Option<u64> {
    let forrest = input
        .lines()
        .map(|row| row.bytes().map(|b| (b - b'0') as u64).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let r = forrest.len();
    let c = forrest[0].len();

    (0..r)
        .cartesian_product(0..c)
        .map(|(r, c)| count_distances(&forrest, r, c))
        .max()
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
