advent_of_code::solution!(13);

#[derive(Debug, Clone, Eq)]
enum Element {
    Integer(i64),
    List(Vec<Element>),
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Integer(i1), Self::Integer(i2)) => i1.cmp(i2),
            (Self::List(v1), Self::List(v2)) => {
                for (e1, e2) in v1.iter().zip(v2.iter()) {
                    if matches!(e1.cmp(e2), Ordering::Equal) {
                        continue;
                    }
                    return e1.cmp(e2);
                }

                v1.len().cmp(&v2.len())
            }

            (Self::Integer(_), Self::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Self::List(_), Self::Integer(_)) => self.cmp(&Self::List(vec![other.clone()])),
        }
    }
}

use std::cmp::Ordering;

// [[[3,[8,10],3,7],[],[[6]]],[[[],[],5,9],1,[[5,10,10,5]]]]
use peg::parser;
parser! {
    pub grammar parser() for str {
        pub rule root_element() -> Element
        = e:(list_element()) {e}

        rule single_element() -> Element
        = e:(list_element() / int_element()) { e }

        rule list_element() -> Element
        = "[" elements:(single_element() ** ",") "]" { Element::List(elements) }

        rule int_element() -> Element
        = num:$(['0'..='9']+) { Element::Integer(num.parse::<i64>().unwrap()) }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let v = input
        .split("\n\n")
        .enumerate()
        .filter_map(|(idx, group)| {
            let (first, second) = group.split_once('\n').unwrap();
            let first = parser::root_element(first.trim()).unwrap();
            let second = parser::root_element(second.trim()).unwrap();
            if first < second {
                Some(idx as u64 + 1)
            } else {
                None
            }
        })
        .sum();

    Some(v)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut v = input
        .split("\n\n")
        .flat_map(|group| {
            let (first, second) = group.split_once('\n').unwrap();
            let first = parser::root_element(first.trim()).unwrap();
            let second = parser::root_element(second.trim()).unwrap();
            [first, second]
        })
        .collect::<Vec<_>>();

    let dividor1 = parser::root_element("[[2]]").unwrap();
    let dividor2 = parser::root_element("[[6]]").unwrap();
    v.push(dividor1.clone());
    v.push(dividor2.clone());

    v.sort();

    let idx1 = v
        .iter()
        .enumerate()
        .find_map(|(idx, ele)| {
            if *ele == dividor1 {
                Some(idx + 1)
            } else {
                None
            }
        })
        .unwrap() as u64;

    let idx2 = v
        .iter()
        .enumerate()
        .find_map(|(idx, ele)| {
            if *ele == dividor2 {
                Some(idx + 1)
            } else {
                None
            }
        })
        .unwrap() as u64;

    Some(idx1 * idx2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let s = "[]";
        assert_eq!(parser::root_element(s).unwrap(), Element::List(vec![]));

        let s = "[[]]";
        assert_eq!(
            parser::root_element(s).unwrap(),
            Element::List(vec![Element::List(vec![])])
        );

        let s = "[[[1,2]]]";
        assert_eq!(
            parser::root_element(s).unwrap(),
            Element::List(vec![Element::List(vec![Element::List(vec![
                Element::Integer(1),
                Element::Integer(2)
            ])])])
        );

        let s = "[[[1,2]],1,2]";
        assert_eq!(
            parser::root_element(s).unwrap(),
            Element::List(vec![
                Element::List(vec![Element::List(vec![
                    Element::Integer(1),
                    Element::Integer(2)
                ])]),
                Element::Integer(1),
                Element::Integer(2)
            ])
        );
        let s = "[[[],[5],6,[7,1,0,8]],[5,1,10],[],[[8,3,0,[5,10],5],1,10,[10,[5,5],10,1,[7,3,9,5]],5],[1,[[10],8,[9,1],[2,3,8,3],[10,0,2]]]]";
        parser::root_element(s).unwrap();
    }

    #[test]
    fn test_part_one() {
        //    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        //    assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
