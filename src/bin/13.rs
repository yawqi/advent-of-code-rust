advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let arrived_time = lines
        .by_ref()
        .take(1)
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let buses = lines
        .take(1)
        .flat_map(|l| l.split(','))
        .filter_map(|x| {
            if x.chars().take(1).next().unwrap() == 'x' {
                None
            } else {
                Some(x.parse::<u64>().unwrap())
            }
        })
        .collect::<Vec<_>>();

    if let Some((id, time)) = (arrived_time..).find_map(|time| {
        if let Some(&id) = buses.iter().find(|&busid| time % busid == 0) {
            Some((id, time))
        } else {
            None
        }
    }) {
        Some((time - arrived_time) * id)
    } else {
        unreachable!()
    }
}

#[derive(Debug, Copy, Clone)]
struct Equation {
    modulus: u64,
    remainder: u64,
    i_mod: Option<u64>,
    i_mod_inv: Option<u64>,
}

impl Equation {
    pub fn new(modulus: u64, remainder: u64) -> Self {
        Self {
            modulus,
            remainder,
            i_mod: None,
            i_mod_inv: None,
        }
    }

    pub fn fill(&mut self, all_m_prod: u64) -> u64 {
        let i_mod = all_m_prod / self.modulus;
        self.i_mod = Some(i_mod);
        self.i_mod_inv = Some(modular_pow(i_mod, self.modulus as u32 - 2, self.modulus));
        self.remainder * self.i_mod.unwrap() * self.i_mod_inv.unwrap()
    }

    pub fn get_modulus(&self) -> u64 {
        self.modulus
    }
}

#[derive(Copy, Clone, Debug)]
struct CRTSolver {
    sums: u64,
    muls: u64,
}

impl CRTSolver {
    pub fn new(mut equations: Vec<Equation>) -> Self {
        let muls = equations
            .iter()
            .fold(1, |prod, equation| prod * equation.get_modulus());

        let sums = equations
            .iter_mut()
            .fold(0, |sum, equation| sum + equation.fill(muls));

        Self { muls, sums }
    }

    pub fn get_minimal(&self) -> u64 {
        self.sums % self.muls
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = input
        .lines()
        .skip(1)
        .take(1)
        .flat_map(|l| l.split(','))
        .enumerate()
        .filter_map(|(idx, x)| {
            if x.chars().take(1).next().unwrap() == 'x' {
                None
            } else {
                let modulus = x.parse::<u64>().unwrap();
                Some(Equation::new(modulus, modulus - idx as u64 % modulus))
            }
        })
        .collect::<Vec<_>>();

    let solver = CRTSolver::new(equations);

    Some(solver.get_minimal())
}

fn modular_pow(x: u64, exp: u32, modulo: u64) -> u64 {
    (match x.checked_pow(exp) {
        Some(x) => x,
        None => {
            let exp_a = exp / 2;
            let exp_b = exp - exp_a;
            modular_pow(x, exp_a, modulo) * modular_pow(x, exp_b, modulo)
        }
    }) % modulo
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

    #[test]
    fn test_pow() {}
}
