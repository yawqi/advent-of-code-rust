use std::collections::{HashMap, HashSet};

advent_of_code::solution!(18);

fn parse_input(input: &str) -> Vec<[i32; 3]> {
    input
        .lines()
        .flat_map(
            |line| 
                line
                    .split(',')
                    .map(|num| num.parse::<i32>().unwrap() * 2)
                    .collect::<Vec<i32>>().
                    try_into()
        )
        .collect()
}
pub fn part_one(input: &str) -> Option<u64> {
    let cords = parse_input(input);
    let offsets = [
        [-1, 0, 0],
        [1, 0, 0],
        [0, -1, 0],
        [0, 1, 0],
        [0, 0, -1],
        [0, 0, 1],
    ];

    let mut counts = HashMap::new();
    for cord in cords {
        for offset in offsets {
            let new_cord = [
                cord[0] + offset[0],
                cord[1] + offset[1],
                cord[2] + offset[2],
            ];
            *counts.entry(new_cord).or_insert(0) += 1;
        }
    }

    Some(
        counts
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .count() as u64
    )

}

fn get_edges(cords: &HashSet<[i32; 3]>) -> HashSet<[i32; 3]> {
    let offsets = [
        [-1, 0, 0],
        [1, 0, 0],
        [0, -1, 0],
        [0, 1, 0],
        [0, 0, -1],
        [0, 0, 1],
    ];

    let mut counts = HashMap::new();
    for cord in cords {
        for offset in offsets {
            let new_cord = [
                cord[0] + offset[0],
                cord[1] + offset[1],
                cord[2] + offset[2],
            ];
            *counts.entry(new_cord).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(k, _)| k)
        .collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let cords = parse_input(input);
    let nexts = [
        [2, 0, 0],
        [-2, 0, 0],
        [0, 2, 0],
        [0, -2, 0],
        [0, 0, 2],
        [0, 0, -2],
    ];

    let mut min_cord = [i32::MAX; 3];
    let mut max_cord = [i32::MIN; 3];

    for cord in &cords {
        for i in 0..3 {
            min_cord[i] = min_cord[i].min(cord[i]);
            max_cord[i] = max_cord[i].max(cord[i]);
        }
    }

    min_cord.iter_mut().for_each(|x| *x -= 2);
    max_cord.iter_mut().for_each(|x| *x += 2);


    let lava_set = cords.clone().into_iter().collect::<std::collections::HashSet<_>>();
    let mut air_set = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::from([min_cord]);
    air_set.insert(min_cord);

    while let Some(cord) = queue.pop_front() {
        let next_cords = nexts
            .iter()
            .map(|mov| [cord[0] + mov[0], cord[1] + mov[1], cord[2] + mov[2]])
            .filter(|next_cord| 
                next_cord[0] >= min_cord[0] && next_cord[0] <= max_cord[0]
                && next_cord[1] >= min_cord[1] && next_cord[1] <= max_cord[1]
                && next_cord[2] >= min_cord[2] && next_cord[2] <= max_cord[2]
                && !air_set.contains(next_cord)
                && !lava_set.contains(next_cord)
            )
            .collect::<Vec<_>>();

        for cord in next_cords {
            if air_set.contains(&cord) {
                continue;
            }
            queue.push_back(cord);
            air_set.insert(cord);
        }
    }

    let air_edges = get_edges(&air_set);
    let lava_edges = get_edges(&lava_set);

    Some(air_edges.intersection(&lava_edges).count() as u64)
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
