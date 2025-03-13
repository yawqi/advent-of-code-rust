advent_of_code::solution!(16);

use std::collections::{btree_map::Entry, HashMap};

// Valve SY has flow rate=0; tunnels lead to valves GW, LW
// Valve TS has flow rate=0; tunnels lead to valves CC, OP
// Valve LU has flow rate=0; tunnels lead to valves PS, XJ
//
use peg::parser;
parser! {
    pub grammar valve() for str {
        pub rule valve() -> Valve
        = "Valve " name:name() " has flow rate=" rate:rate() "; tunnels lead to valves " links:(name() ** ",")
        {
            Valve {
                name,
                rate,
                links
            }
        }

        rule name() -> [u8; 2]
        = name:$(['A'..='Z']*<2,2>) {
            let mut ret = [0u8, 0u8];
            ret[0] = name.as_bytes()[0] as u8;
            ret[1] = name.as_bytes()[1] as u8;
            ret
        }

        rule rate() -> u64
        = rate:$(['0'..='9']+) { rate.parse::<u64>().unwrap() }
    }
}
type Name = [u8; 2];
type Path = Vec<(Name, Name)>;

#[derive(Debug, Clone)]
struct Valve {
    name: Name,
    rate: u64,
    links: Vec<Name>,
}

#[derive(Debug, Clone)]
struct Network {
    network: HashMap<Name, (Valve, Vec<(Name, Path)>)>,
}

impl Network {
    pub fn new(valves: Vec<Valve>) -> Self {
        let network = valves
            .into_iter()
            .map(|v| (v.name.clone(), v))
            .collect::<HashMap<_, _>>();

        let network = network
            .iter()
            .map(|(name, valve)| {
                (
                    name.clone(),
                    (valve.clone(), Self::connections(name, &network)),
                )
            })
            .collect();

        Self { network }
    }

    pub fn connections(start: &Name, network: &HashMap<Name, Valve>) -> Vec<(Name, Path)> {
        let mut ret = vec![];
        let mut curr = HashMap::new();
        let mut next = HashMap::new();

        curr.insert(start.clone(), vec![]);
        while !curr.is_empty() {
            for (name, path) in curr {
                if let Entry::Vacant(entry) =
            }
            curr = next;
        }

        ret
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
