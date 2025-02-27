advent_of_code::solution!(7);

// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Bag {
    color: String,
    contains: Vec<(String, usize)>,
}

impl Bag {
    pub fn containable(&self, color: &str) -> bool {
        self.contains
            .iter()
            .any(|(bag_color, _)| *bag_color == color)
    }

    pub fn new(color: String, contains: Vec<(String, usize)>) -> Bag {
        Bag { color, contains }
    }
}

peg::parser! {
    grammar bag_parser() for str {
        pub rule bag() -> Bag
        = b: empty_bag() / b: stuffed_bag() { b }
        // = b: stuffed_bag() / b: empty_bag() { b }

        pub rule stuffed_bag() -> Bag
        = c:color() whitespaces() "bags contain" whitespaces() bags: contained_bags() "."? { Bag::new(c.to_owned(), bags)}

        rule empty_bag() -> Bag
        = c:color() whitespaces() "bags contain no other bags." { Bag::new(c.to_owned(), vec![])}

        rule color() -> String
        = c:$((['a'..='z' | 'A'..='Z']*)** <2,2> whitespaces()) { c.to_owned() }
        // = c:$(['a'..='z' | 'A'..='Z']* whitespaces() ['a'..='z' | 'A'..='Z']*) { dbg!(c).to_owned() }

        rule whitespaces()
        = [ c if c.is_whitespace() ] *

        rule count() -> usize
        = num:$(['0'..='9']*) {? num.parse::<usize>().or(Err("usize")) }

        rule contained_bags() -> Vec<(String, usize)>
        = bags: (single_bag() ** ", " ) { bags }

        rule single_bag() -> (String, usize)
        = count:count() whitespaces() color:color() whitespaces() "bag" "s"? { (color, count) }
    }
}

// impl<'input> FromStr for Bag<'input> {
//     type Err = anyhow::Error;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {}
// }

use std::collections::HashSet;
pub fn part_one(input: &str) -> Option<u64> {
    let bags = input
        .lines()
        .map(|l| bag_parser::bag(l).unwrap())
        .collect::<Vec<_>>();

    let mut usable_bags = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back("shiny gold");

    while let Some(target) = queue.pop_front() {
        for bag in &bags {
            if !usable_bags.contains(&bag.color) && bag.containable(target) {
                usable_bags.insert(bag.color.clone());
                queue.push_back(&bag.color);
            }
        }
    }

    Some(usable_bags.len() as u64)
}

fn count_bag(bags: &HashMap<String, Bag>, target: &str) -> usize {
    if let Some(target_bag) = bags.get(target) {
        let mut ret = 1;
        for (next_bag, count) in &target_bag.contains {
            ret += count * count_bag(bags, next_bag);
        }
        ret
    } else {
        0
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let bags = input
        .lines()
        .map(|l| {
            let bag = bag_parser::bag(l).unwrap();
            (bag.color.clone(), bag)
        })
        .collect::<HashMap<_, _>>();

    Some(count_bag(&bags, "shiny gold") as u64 - 1)
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
