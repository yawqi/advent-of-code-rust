advent_of_code::solution!(9);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = input
        .lines()
        .map(str::parse::<u64>)
        .map(Result::into_iter)
        .flatten()
        .collect::<Vec<_>>();

    let mut previous = numbers.iter().take(25).fold(HashMap::new(), |mut map, v| {
        let e = map.entry(*v).or_insert(0);
        *e += 1;
        map
    });

    let mut available = (0..24)
        .into_iter()
        .map(|idx| {
            std::iter::repeat(numbers.iter().skip(idx).take(1).next().unwrap())
                .take(25 - idx - 1)
                .zip(numbers.iter().skip(idx + 1))
                .filter_map(|(v1, v2)| if *v1 == *v2 { None } else { Some(*v1 + *v2) })
        })
        .flatten()
        .fold(HashMap::new(), |mut map, v| {
            let e = map.entry(v).or_insert(0);
            *e += 1;
            map
        });

    let target = numbers
        .iter()
        .zip(numbers.iter().skip(25))
        .find(|(&to_remove, &to_add)| {
            if !available.contains_key(&to_add) {
                return true;
            }

            previous.entry(to_remove).and_modify(|v| *v -= 1);
            if *previous.get(&to_remove).unwrap() == 0 {
                previous.remove(&to_remove);
            }

            for (k, v) in previous.iter() {
                if *k != to_remove {
                    let num = *k + to_remove;
                    available.entry(num).and_modify(|count| *count -= *v);
                    if *available.get(&num).unwrap() == 0 {
                        available.remove(&num);
                    }
                }

                if *k != to_add {
                    let num = *k + to_add;
                    *(available.entry(num).or_insert(0)) += *v;
                }
            }
            *(previous.entry(to_add).or_insert(0)) += 1;

            false
        });
    Some(*target.unwrap().1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let target = 1038347917u64;

    let numbers = input
        .lines()
        .map(str::parse::<u64>)
        .map(Result::into_iter)
        .flatten()
        .collect::<Vec<_>>();

    let mut map = HashMap::new();
    let mut sum = 0;
    map.insert(0, 0);
    let (idx, _) = numbers
        .iter()
        .enumerate()
        .find(|(idx, val)| {
            sum += *val;
            if sum > target {
                if map.contains_key(&(sum - target)) {
                    return true;
                }
            }
            map.insert(sum, idx + 1);
            false
        })
        .unwrap();

    let pidx = *map.get(&(sum - target)).unwrap();
    let min = *numbers[pidx..=idx].iter().min().unwrap();
    let max = *numbers[pidx..=idx].iter().max().unwrap();
    Some(min + max)
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
