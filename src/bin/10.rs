use std::str::FromStr;

use anyhow::Ok;

advent_of_code::solution!(10);

// Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. *What is the sum of these six signal strengths?*
#[derive(Debug, Clone, Copy)]
enum Inst {
    Noop,
    Add(i8),
}

impl Inst {
    pub fn cycles(&self) -> u64 {
        match *self {
            Self::Noop => 1,
            Self::Add(_) => 2,
        }
    }

    pub fn apply<const N: usize, const M: usize>(&self, mach: &mut Machine<N, M>) {
        match self {
            Self::Noop => {}
            Self::Add(v) => mach.add_reg(*v),
        }
    }
}

impl FromStr for Inst {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Self::Noop)
        } else {
            let (_, count) = s.split_once(' ').unwrap();
            let count = count.parse::<i8>().unwrap();
            Ok(Self::Add(count))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RunningInst {
    inst: Inst,
    cycle_to_done: u64,
}

impl RunningInst {
    pub fn new(inst: Inst, curr_cycle: u64) -> Self {
        Self {
            inst,
            cycle_to_done: curr_cycle + inst.cycles(),
        }
    }

    pub fn apply<const N: usize, const M: usize>(&self, mach: &mut Machine<N, M>) {
        self.inst.apply(mach);
    }
}

#[derive(Debug)]
struct Machine<const N: usize, const M: usize> {
    x_reg: i64,
    cycle: u64,
    unfinished: RunningInst,

    recorded_sum: i64,
}

impl<const N: usize, const M: usize> Machine<N, M> {
    pub fn new() -> Self {
        Self {
            x_reg: 1,
            cycle: 0,
            unfinished: RunningInst::new(Inst::Noop, 0),
            recorded_sum: 0,
        }
    }

    pub fn add_reg(&mut self, x: i8) {
        self.x_reg += x as i64;
    }

    pub fn execute(&mut self, inst: Inst) {
        self.unfinished = RunningInst::new(inst, self.cycle);
        while self.unfinished.cycle_to_done > self.cycle {
            self.cycle += 1;
            if self.cycle % 40 == 20 {
                self.record();
            }
            self.draw();
        }
        self.unfinished.clone().apply(self);
    }

    fn record(&mut self) {
        self.recorded_sum += self.cycle as i64 * self.x_reg;
    }

    fn is_sprite_in_range(&self) -> bool {
        self.cycle as i64 % 40 >= self.x_reg && self.cycle as i64 % 40 <= self.x_reg + 2
    }

    fn draw(&self) {
        if self.is_sprite_in_range() {
            print!("#");
        } else {
            print!(".");
        }

        if self.cycle % N as u64 == 0 {
            println!();
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut mach = Machine::<40, 6>::new();
    input.lines().flat_map(str::parse::<Inst>).for_each(|inst| {
        mach.execute(inst);
    });

    Some(mach.recorded_sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut mach = Machine::<40, 6>::new();
    input.lines().flat_map(str::parse::<Inst>).for_each(|inst| {
        mach.execute(inst);
    });
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
