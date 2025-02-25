advent_of_code::solution!(3);

fn get_trees(map: &[&[u8]], slope: (usize, usize)) -> u64 {
    let (row_diff, col_diff) = slope;

    (0..)
        .filter(|r| *r % row_diff == 0)
        .take_while(|r| *r < map.len())
        .zip((0..).filter(|c| *c % col_diff == 0))
        .fold(0, |sum: u64, (ridx, cidx)| {
            match map[ridx][cidx % map[0].len()] {
                b'#' => sum + 1,
                _ => sum,
            }
        })

    //let mut cidx = 0;
    //let ref_cidx = &mut cidx;
    //map.iter()
    //    .enumerate()
    //    .filter_map(|(idx, row)| if idx % row_diff == 0 { Some(row) } else { None })
    //    .fold(0, |count, row| {
    //        let v = match row.get(*ref_cidx) {
    //            None | Some(b'.') => count,
    //            _ => count + 1,
    //        };
    //        *ref_cidx = (*ref_cidx + col_diff) % row.len();
    //        v
    //    }) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    Some(get_trees(&map, (1, 3)))
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let query = [(1usize, 1usize), (1, 3), (1, 5), (1, 7), (2, 1)];
    Some(query.into_iter().map(|q| get_trees(&map, q)).product())
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
