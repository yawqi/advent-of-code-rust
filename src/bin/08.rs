advent_of_code::solution!(8);

// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6

#[derive(Debug, Copy, Clone)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug, Clone)]
struct Machine {
    pub ops: Vec<(Op, bool)>,
    pub acc: i32,
    pub pc: i32,
}

impl Machine {
    fn run(&mut self) -> u64 {
        loop {
            let (op, runned) = &mut self.ops[usize::try_from(self.pc).unwrap()];
            if *runned {
                break;
            }

            *runned = true;
            match op {
                Op::Acc(operand) => self.acc += *operand,
                Op::Jmp(operand) => {
                    self.pc += *operand;
                    continue;
                }
                _ => {}
            }
            self.pc += 1;
        }
        self.acc as u64
    }

    fn try_run(&mut self) -> Option<u64> {
        self.ops
            .iter_mut()
            .for_each(|(_, visited)| *visited = false);
        self.pc = 0;
        self.acc = 0;
        loop {
            if self.pc >= self.ops.len() as i32 {
                return Some(self.acc as u64);
            }

            let (op, runned) = &mut self.ops[usize::try_from(self.pc).unwrap()];
            if *runned {
                return None;
            }

            *runned = true;
            match op {
                Op::Acc(operand) => self.acc += *operand,
                Op::Jmp(operand) => {
                    self.pc += *operand;
                    continue;
                }
                _ => {}
            }
            self.pc += 1;
        }
    }

    fn swap_op(&mut self, index: usize) -> Option<Op> {
        match self.ops[index].0 {
            Op::Nop(operand) => Some(Op::Jmp(operand)),
            Op::Jmp(operand) => Some(Op::Nop(operand)),
            _ => None,
        }
    }

    fn find_it(&mut self) -> u64 {
        for i in 0..self.ops.len() {
            if let Some(replacement) = self.swap_op(i) {
                let origin = self.ops[i].0;
                self.ops[i].0 = replacement;
                if let Some(res) = self.try_run() {
                    return res;
                }
                self.ops[i].0 = origin;
            }
        }
        unreachable!()
    }
}

peg::parser! {
    grammar op_parser() for str {
        pub rule op() -> crate::Op
        = code:code() whitespaces() count:count() {
            match code {
                "acc" => crate::Op::Acc(count),
                "nop" => crate::Op::Nop(count),
                "jmp" => crate::Op::Jmp(count),
                _ => todo!(),
            }
        }

        rule code() -> &'input str
        = code:$("acc" / "nop" / "jmp") { code }

        rule whitespaces()
        = ['\n' | '\t' | '\r' | ' ']*

        rule count() -> i32
        = mark:$(['+' | '-']) num:$(['0'..='9']*) {
            let neg = mark.starts_with('-');
            let ret = num.parse::<i32>().unwrap();
            if neg {
                -ret
            } else {
                ret
            }
        }
    }

}
pub fn part_one(input: &str) -> Option<u64> {
    let ops = input
        .lines()
        .map(|line| {
            let op = op_parser::op(line).unwrap();
            (op, false)
        })
        .collect::<Vec<_>>();

    let mut machine = Machine { ops, acc: 0, pc: 0 };
    Some(machine.run())
}

pub fn part_two(input: &str) -> Option<u64> {
    let ops = input
        .lines()
        .map(|line| {
            let op = op_parser::op(line).unwrap();
            (op, false)
        })
        .collect::<Vec<_>>();

    let mut machine = Machine { ops, acc: 0, pc: 0 };

    Some(machine.find_it())
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
