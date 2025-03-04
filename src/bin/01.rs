use std::cmp::Reverse;
use std::collections::BinaryHeap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    input
        .split("\n\n")
        .map(|foods| foods.lines().flat_map(str::parse::<u64>).sum())
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut min_heap = BinaryHeap::new();

    input
        .split("\n\n")
        .map(|foods| foods.lines().flat_map(str::parse::<u64>).sum())
        .for_each(|calories: u64| {
            if min_heap.len() < 3 {
                min_heap.push(Reverse(calories));
            } else if calories > min_heap.peek().unwrap().0 {
                min_heap.pop();
                min_heap.push(Reverse(calories));
            }
        });

    Some(min_heap.into_iter().fold(0, |sum, rev| sum + rev.0))
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
