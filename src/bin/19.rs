use std::{collections::{VecDeque, HashSet}, ops::{Index, IndexMut}, str::FromStr};
use std::hash::{Hash, Hasher};

use chumsky::prelude::*;
use rayon::prelude::*;

advent_of_code::solution!(19);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Minerals {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Minerals {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ore" => Ok(Minerals::Ore),
            "clay" => Ok(Minerals::Clay),
            "obsidian" => Ok(Minerals::Obsidian),
            "geode" => Ok(Minerals::Geode),
           _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct MineralCounts([i32; 4]);

impl MineralCounts {
    pub fn containable(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a >= b)
    }
}

impl From<[i32; 4]> for MineralCounts {
    fn from(value: [i32; 4]) -> Self {
        let mut ret = Self::default();
        ret.0 = value;
        ret
    }
}

impl std::ops::Index<Minerals> for MineralCounts {
    type Output = i32;

    fn index(&self, index: Minerals) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl std::ops::IndexMut<Minerals> for MineralCounts {
    fn index_mut(&mut self, index: Minerals) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl std::ops::AddAssign<MineralCounts> for MineralCounts {
    fn add_assign(&mut self, rhs: MineralCounts) {
        for i in 0..4 {
            self.0[i] += rhs.0[i];
        }
    }
}

impl std::ops::SubAssign<MineralCounts> for MineralCounts {
    fn sub_assign(&mut self, rhs: MineralCounts) {
        for i in 0..4 {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl std::ops::MulAssign<usize> for MineralCounts {
    fn mul_assign(&mut self, rhs: usize) {
        for i in 0..4 {
            self.0[i] *= rhs as i32;
        }
    }
}

impl std::ops::Mul<usize> for MineralCounts {
    type Output = MineralCounts;
    fn mul(self, rhs: usize) -> MineralCounts {
        MineralCounts(
            [
                self.0[0] * rhs as i32,
                self.0[1] * rhs as i32,
                self.0[2] * rhs as i32,
                self.0[3] * rhs as i32,
            ]
        )
   }
}

impl std::ops::Add for MineralCounts {
    type Output = MineralCounts;
    fn add(self, rhs: MineralCounts) -> MineralCounts {
        let mut ret = self;
        ret += rhs;
        ret
    }
}

impl std::ops::Sub for MineralCounts {
    type Output = MineralCounts;
    fn sub(self, rhs: MineralCounts) -> MineralCounts {
        let mut ret = self;
        ret -= rhs;
        ret
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Blueprint {
    pub id: i32,
    pub robot_costs: [MineralCounts; 4],
}

impl Index<Minerals> for Blueprint {
    type Output = MineralCounts;

    fn index(&self, index: Minerals) -> &Self::Output {
        &self.robot_costs[index as usize]
    }
}

impl IndexMut<Minerals> for Blueprint {
    fn index_mut(&mut self, index: Minerals) -> &mut Self::Output {
        &mut self.robot_costs[index as usize]
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Factory {
    blueprint: Blueprint,
    remainging_time: usize,
    robots: MineralCounts,
    inventory: MineralCounts,
    productions: MineralCounts,
}


impl Hash for Factory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robots.hash(state);
        self.inventory.hash(state);
        self.remainging_time.hash(state);
    }
}

impl PartialEq for Factory {
    fn eq(&self, other: &Self) -> bool {
        self.inventory == other.inventory && self.robots == other.robots && self.remainging_time == other.remainging_time
    }
}

impl Eq for Factory {}

impl Factory {
    pub fn new(blueprint: Blueprint, time: usize) -> Self {
        Self {
            blueprint,
            robots: [1, 0, 0, 0].into(),
            inventory: [0, 0, 0, 0].into(),
            productions: [0, 0, 0, 0].into(),
            remainging_time: time,
        }

    }

    pub fn produce_robot(&self, robot: Minerals) -> Option<Factory> {
        let maximum = self.inventory + self.robots * self.remainging_time;
        let robot_cost = self.blueprint[robot];

        let minimum = self.inventory - self.robots;
        if minimum.containable(&robot_cost) {
            return None;
        }

        if !maximum.containable(&robot_cost) {
            None
        } else {
            let mut new_factory = *self;
            while !new_factory.inventory.containable(&robot_cost) && new_factory.remainging_time > 0 {
                new_factory.tick();
            }
            new_factory.robots[robot] += 1;
            new_factory.inventory -= robot_cost;
            Some(new_factory)
        }
    }

    pub fn tick(&mut self) {
        self.inventory += self.robots;
        self.productions += self.robots;
        self.remainging_time -= 1;
    }

    pub fn finish(&self) -> Factory {
        let mut factory = self.clone();
        while factory.remainging_time > 0 {
            factory.tick();
        }
        factory
    }

    pub fn get_geode_count(&self) -> u64 {
        self.inventory[Minerals::Geode] as u64
    }

    pub fn get_max_geode_count(&self) -> u64 {
        self.inventory[Minerals::Geode] as u64 + (self.robots[Minerals::Geode] as u64 + self.remainging_time as u64 / 2 ) * self.remainging_time as u64
    }
}

struct Simulation {
    blueprint: Blueprint,
    factories: VecDeque<Factory>,
    factory_set: HashSet<Factory>,
    max_geodes: u64,
}

impl Simulation {
    pub fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            factories: VecDeque::new(),
            factory_set: HashSet::new(),
            max_geodes: 0,
        }
    }
    pub fn get_grades(&self) -> u64 {
        self.blueprint.id as u64 * self.max_geodes
    }

    pub fn run(&mut self, time: usize) {
        let fac = Factory::new(self.blueprint, time);
        let robot_kind = [Minerals::Ore, Minerals::Clay, Minerals::Obsidian, Minerals::Geode];
        self.factories.push_back(fac);
        self.factory_set.insert(fac);

        while let Some(fac) = self.factories.pop_front() {
            // println!("{:?}", fac);
            let mut is_done = true;
            if fac.remainging_time == 0 {
                self.max_geodes = self.max_geodes.max(fac.get_geode_count());
                continue;
            }

            for kind in robot_kind {
                if let Some(next_fac) = fac.produce_robot(kind) {
                    // println!("\tnxt: {:?}", next_fac);
                    is_done = false;
                    if !self.factory_set.contains(&next_fac) && self.max_geodes < next_fac.get_max_geode_count() {
                        self.factory_set.insert(next_fac);
                        self.factories.push_back(next_fac);
                    }
                }
            }

            if is_done {
                let fac = fac.finish();
                self.max_geodes = self.max_geodes.max(fac.get_geode_count());
            }

        }
    }
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 3 ore and 8 obsidian.
fn bp_parser<'src>() -> impl Parser<'src, &'src str, Blueprint> {
    let number = text::int(10)
        .map(|s: &str| s.parse::<i32>().unwrap())
        .padded();

    let index = just("Blueprint ")
        .ignore_then(number)
        .then_ignore(just(": "));

    let mineral = choice((
        just("ore").padded().to(Minerals::Ore),
        just("clay").padded().to(Minerals::Clay),
        just("obsidian").padded().to(Minerals::Obsidian),
        just("geode").padded().to(Minerals::Geode),
    ));

    let costs = number
        .then(mineral)
        .separated_by(just("and").padded())
        .collect::<Vec<(i32, Minerals)>>()
        .map(|costs| {
            let mut counts = MineralCounts::default();
            for (count, mineral) in costs {
                counts[mineral] = count;
            }
            counts
        });

    let robot_recipe = just("Each").padded()
        .ignore_then(mineral)
        .then_ignore(just("robot costs").padded())
        .then(costs)
        .then_ignore(just(".").padded());

    let bp = index
        .then(robot_recipe.repeated().collect::<Vec<_>>())
        .map(|(id, recipes)| {
            let mut bp = Blueprint::default();
            bp.id = id;
            for (mineral, recipe) in recipes {
                bp[mineral] = recipe;
            }
            bp
        });
    bp
}

pub fn part_one(input: &str) -> Option<u64> {
    let bps = input
        .lines()
        .map(|l| bp_parser().parse(l).unwrap())
        .collect::<Vec<_>>();

    let remaining_time = 24usize;
    let mut grades = 0;

    let grades = bps.par_iter().map(|bp| {
        let mut simu = Simulation::new(bp.clone());
        simu.run(remaining_time);
        let grade = simu.get_grades();
        println!("{} {}", bp.id, grade);
        grade
    }).sum();

    // for bp in bps {
    //     let mut simu = Simulation::new(bp);
    //     simu.run(remaining_time);
    //     let grade = simu.get_grades();
    //     println!("{} {}", bp.id, grade);
    //     grades += grade;
    // }

    Some(grades)
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
