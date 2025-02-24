use std::collections::HashSet;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut set = HashSet::new();
    input
        .lines()
        .map(|v| v.trim().parse::<u64>().unwrap())
        .find_map(|v| {
            if set.contains(&(2020 - v)) {
                Some(v * (2020 - v))
            } else {
                set.insert(v);
                None
            }
        })
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut nums = input
        .lines()
        .map(|v| v.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();
    let mut head = 0usize;
    let mut tail = nums.len() - 1;

    while head < tail - 1 {
        let head_v = nums[head];
        let mut tail_v = nums[tail];
        if head_v + tail_v + head_v > 2020 {
            tail -= 1;
            continue;
        }

        if head_v + tail_v + tail_v < 2020 {
            head += 1;
            continue;
        }

        let mut middle = head + 1;
        let mut middle_v = nums[middle];
        let saved_tail = tail;
        while middle < tail {
            let target = 2020 - head_v - tail_v;
            match middle_v {
                v if v == target => {
                    return Some(head_v * middle_v * tail_v);
                }
                v if v > target => {
                    tail -= 1;
                    tail_v = nums[tail];
                }
                v if v < target => {
                    middle += 1;
                    middle_v = nums[middle];
                }
                _ => unreachable!(),
            }
        }
        tail = saved_tail;
        head += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
