use std::collections::HashMap;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let mut nums = input
        .lines()
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    nums.push(0);
    nums.sort();
    nums.push(nums.last().unwrap() + 3);

    let counts = nums
        .windows(2)
        .map(|w| w[1] - w[0])
        .fold((0, 0), |mut counts, diff| {
            match diff {
                1 => counts.0 += 1,
                3 => counts.1 += 1,
                _ => unreachable!(),
            }
            counts
        });

    Some(counts.0 * counts.1)
}

fn get_path_count(curr: u64, slice: &[u64], map: &mut HashMap<u64, u64>) -> u64 {
    if slice.len() <= 1 {
        return 1;
    }

    if map.contains_key(&curr) {
        return *map.get(&curr).unwrap();
    }

    let count = slice
        .iter()
        .enumerate()
        .take_while(|(_, v)| curr + 3 >= **v && **v > curr)
        .fold(0, |count, (idx, v)| {
            count + get_path_count(*v, &slice[idx + 1..], map)
        });

    map.insert(curr, count);
    count
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut nums = input
        .lines()
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    nums.push(0);
    nums.sort();
    let mut map = HashMap::new();
    Some(get_path_count(nums[0], &nums[1..], &mut map))
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
