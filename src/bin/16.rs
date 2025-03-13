advent_of_code::solution!(16);

use core::str;
use itertools::Itertools;
use peg::parser;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
};

// Valve SY has flow rate=0; tunnels lead to valves GW, LW
// Valve TS has flow rate=0; tunnels lead to valves CC, OP
// Valve LU has flow rate=0; tunnels lead to valves PS, XJ
//

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Name([u8; 2]);
impl Name {
    pub fn new(name: [u8; 2]) -> Self {
        Self(name)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { str::from_utf8_unchecked(&self.0) })
    }
}

#[derive(Clone, Debug)]
struct Path(Vec<(Name, Name)>);
impl Path {
    pub fn new(v: Vec<(Name, Name)>) -> Self {
        Self(v)
    }

    pub fn push(&mut self, step: (Name, Name)) {
        self.0.push(step);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (src, dest) in &self.0 {
            write!(f, "from {src} to {dest};")?;
        }
        Ok(())
    }
}

parser! {
    pub grammar valve() for str {
        pub rule valve() -> Valve
        = "Valve " name:name() " has flow rate=" rate:rate() ("; tunnel leads " / "; tunnels lead ") "to " ("valve " / "valves ") links:(name() ** ", ")
        {
            Valve {
                name,
                rate,
                links
            }
        }

        rule name() -> Name
        = name:$(['A'..='Z']*<2,2>) {
            let mut ret = [0u8, 0u8];
            ret[0] = name.as_bytes()[0];
            ret[1] = name.as_bytes()[1];
            Name(ret)
        }

        rule rate() -> u64
        = rate:$(['0'..='9']+) { rate.parse::<u64>().unwrap() }
    }
}

#[derive(Debug, Clone)]
struct Valve {
    name: Name,
    rate: u64,
    links: Vec<Name>,
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} has flow rate={}; tunnel leads to {}",
            self.name,
            self.rate,
            self.links.iter().join(", ")
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Network {
    network: HashMap<Name, (Valve, HashMap<Name, Path>)>,
}

impl Network {
    pub fn new(valves: Vec<Valve>) -> Self {
        let network = valves
            .into_iter()
            .map(|v| (v.name, v))
            .collect::<HashMap<_, _, _>>();

        let n = network
            .iter()
            .map(|(name, valve)| (*name, (valve.clone(), Self::connections(name, &network))))
            .collect();

        Self { network: n }
    }

    fn connections(start: &Name, network: &HashMap<Name, Valve>) -> HashMap<Name, Path> {
        let mut paths = HashMap::new();
        let mut curr = HashMap::new();

        curr.insert(*start, Path::new(vec![]));
        while !curr.is_empty() {
            let mut next = HashMap::new();
            for (name, path) in curr {
                network[&name].links.iter().for_each(|next_valve| {
                    if let Entry::Vacant(e) = paths.entry(*next_valve) {
                        let mut next_path = path.clone();
                        next_path.push((name, *next_valve));
                        e.insert(next_path.clone());
                        next.insert(*next_valve, next_path);
                    }
                });
            }
            curr = next;
        }
        paths
    }

    pub fn get_connections(&self, name: &Name) -> &HashMap<Name, Path> {
        &(self.network.get(name).unwrap().1)
    }

    pub fn get_valve(&self, name: &Name) -> &Valve {
        &(self.network.get(name).unwrap().0)
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (from, (_, conns)) in &self.network {
            writeln!(f, "\n\nFrom {from}, we can go to:")?;
            for path in conns.values() {
                writeln!(f, "\t{path}")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Move {
    path: Path,
    dest: Valve,
}

impl Move {
    pub fn new(path: Path, dest: Valve) -> Self {
        Self { path, dest }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", &self.dest)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct State {
    released_pressure: u64,
    pressure: u64,
    remaining_time: u64,
    current_position: Name,
    opened_valve: HashSet<Name>,
}

impl State {
    pub fn new(total_time: u64, start_point: Name) -> Self {
        Self {
            released_pressure: 0,
            pressure: 0,
            remaining_time: total_time,
            current_position: start_point,
            opened_valve: HashSet::new(),
        }
    }

    fn apply(&self, mov: Move) -> Option<Self> {
        let time_consumed = mov.path.len() as u64 + 1;
        if self.remaining_time < time_consumed
            || self.opened_valve.contains(&mov.dest.name)
            || mov.dest.rate == 0
        {
            return None;
        }

        let mut opened_valve = self.opened_valve.clone();
        opened_valve.insert(mov.dest.name);
        Some(Self {
            released_pressure: self.released_pressure + self.pressure * time_consumed,
            pressure: self.pressure + mov.dest.rate,
            remaining_time: self.remaining_time - time_consumed,
            current_position: mov.dest.name,
            opened_valve,
        })
    }

    pub fn rewards(&self) -> u64 {
        self.released_pressure + self.remaining_time * self.pressure
    }

    pub fn find_best_state(&self, network: &Network) -> Self {
        let mut best_state = self.clone();

        if self.remaining_time == 0 {
            return self.clone();
        }

        for (dest, path) in network.get_connections(&self.current_position) {
            let dest_valve = network.get_valve(dest).clone();

            let mov = Move::new(path.clone(), dest_valve);
            if let Some(next_state) = self.apply(mov) {
                let state = next_state.find_best_state(network);
                if state.rewards() > best_state.rewards() {
                    best_state = state;
                }
            }
        }

        best_state
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let network = Network::new(
        input
            .lines()
            .map(|line| valve::valve(line.trim()).unwrap())
            .collect_vec(),
    );

    let initial_state = State::new(30, Name::new([b'A', b'A']));
    let best_state = initial_state.find_best_state(&network);

    Some(best_state.rewards())
}

#[derive(Debug, Clone)]
struct DualState {
    human: State,
    elephant: State,
    opened_valve: HashSet<Name>,
}

impl DualState {
    pub fn new(total_time: u64, start_point: Name) -> Self {
        Self {
            human: State::new(total_time, start_point),
            elephant: State::new(total_time, start_point),
            opened_valve: HashSet::new(),
        }
    }

    pub fn apply(&self, moves: [Move; 2]) -> Option<Self> {
        let time_consumed = moves.iter().map(|mov| mov.path.len() as u64 + 1).collect();
        if self.remaining_time < time_consumed || self.opened_valve.contains(&mov.dest.name) {
            return None;
        }

        let mut opened_valve = self.opened_valve.clone();
        opened_valve.insert(mov.dest.name);
        Some(Self {
            released_pressure: self.released_pressure + self.pressure * time_consumed,
            pressure: self.pressure + mov.dest.rate,
            remaining_time: self.remaining_time - time_consumed,
            current_position: mov.dest.name,
            opened_valve,
        })
    }

    pub fn rewards(&self) -> u64 {
        self.elephant.rewards() + self.human.rewards()
    }

    pub fn find_best_state(&self, network: &Network) -> Self {
        let mut best_state = self.clone();

        if self.human.remaining_time == 0 && self.elephant.remaining_time == 0 {
            return self.clone();
        }

        let human_connections = network.get_connections(&self.human.current_position);
        let elephant_connections = network.get_connections(&self.elephant.current_position);

        for (human, elephant) in human_connections
            .into_iter()
            .flat_map(|human| std::iter::repeat(human).zip(elephant_connections.iter()))
        {
            if self.opened_valve.contains(human.0)
                || self.opened_valve.contains(elephant.0)
                || human.0 == elephant.0
            {
                continue;
            }

            let human_valve = network.get_valve(human.0);
            let elephant_valve = network.get_valve(elephant.0);

            let moves = [
                Move::new(human.1.clone(), human_valve.clone()),
                Move::new(elephant.1.clone(), elephant_valve.clone()),
            ];

            if let Some(next_state) = self.apply(moves) {
                let state = next_state.find_best_state(network);
                if state.rewards() > best_state.rewards() {
                    best_state = state;
                }
            }
        }

        best_state
    }
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
