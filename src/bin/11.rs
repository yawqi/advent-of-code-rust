advent_of_code::solution!(11);

use core::str;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::str::FromStr;

// Monkey 0:
//   Starting items: 65, 78
//   Operation: new = old * 3
//   Test: divisible by 5
//     If true: throw to monkey 2
//     If false: throw to monkey 3

use anyhow::anyhow;
use monkey::monkey;
use peg::parser;
parser! {
    pub grammar monkey() for str {
        pub rule monkey() -> Monkey
        = index:index() items:items() operation:operation() test:test() if_ture:if_true() if_false:if_false() {
            Monkey::new(index, items, operation, test, (if_ture, if_false))
        }

        rule index() -> usize
        = "Monkey " num: numbers() ":" whitespaces() { num }

        rule items() -> VecDeque<usize>
        = "Starting items: " items:(numbers() ** ", ") whitespaces() { items.into_iter().collect() }

        rule operation() -> MonkeyOp
        = "Operation: new = old " op:$([c if c != '\n']+) whitespaces() { op.parse::<MonkeyOp>().unwrap() }

        rule test() -> usize
        = "Test: divisible by " num:numbers() whitespaces() { num }

        rule if_true() -> usize
        = "If true: throw to monkey " num:numbers() whitespaces() { num }

        rule if_false() -> usize
        = "If false: throw to monkey " num:numbers() whitespaces()* { num }

        rule numbers() -> usize
        = num:$(['0'..='9']+) { num.parse::<usize>().unwrap() }

        rule whitespaces()
        = ['\n' | ' ' | '\t']+
    }
}

#[derive(Debug, Clone, Copy)]
enum MonkeyOp {
    Add(u64),
    Mul(u64),
    Square,
}

impl FromStr for MonkeyOp {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.ends_with("old") {
            Ok(Self::Square)
        } else {
            let bytes = s.trim().as_bytes();
            let op = bytes[0];
            let number = unsafe { str::from_utf8_unchecked(&bytes[2..]) }
                .parse()
                .unwrap();
            match op {
                b'+' => Ok(Self::Add(number)),
                b'*' => Ok(Self::Mul(number)),
                _ => Err(anyhow!("not valid operations")),
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    index: usize,
    inspections: usize,
    holding_items: VecDeque<usize>,
    operation: MonkeyOp,
    divisiable: usize,
    decision: (usize, usize),
    original: usize,
    to_divide: usize,
    // receivers: (Option<Rc<RefCell<Monkey>>>, Option<Rc<RefCell<Monkey>>>),
}

impl Monkey {
    pub fn new(
        index: usize,
        holding_items: VecDeque<usize>,
        operation: MonkeyOp,
        divisiable: usize,
        decision: (usize, usize),
    ) -> Self {
        Self {
            index,
            inspections: 0,
            original: holding_items.len(),
            holding_items,
            operation,
            divisiable,
            decision,
            to_divide: 3,
        }
    }

    fn catch_an_item(&mut self, item: usize) {
        self.holding_items.push_back(item);
    }

    fn throw_head_item_to(&mut self, receivers: &[Rc<RefCell<Monkey>>]) {
        if self.holding_items.is_empty() {
            return;
        }

        let head_item = self.holding_items.pop_front().unwrap();
        if head_item % self.divisiable == 0 {
            receivers[self.decision.0]
                .borrow_mut()
                .catch_an_item(head_item);
        } else {
            receivers[self.decision.1]
                .borrow_mut()
                .catch_an_item(head_item);
        }
    }

    fn inspect_head_item(&mut self) {
        if self.holding_items.is_empty() {
            return;
        }

        match self.operation {
            MonkeyOp::Add(to_add) => self.holding_items[0] += to_add as usize,
            MonkeyOp::Mul(to_mul) => self.holding_items[0] *= to_mul as usize,
            MonkeyOp::Square => self.holding_items[0] *= self.holding_items[0],
        }

        self.holding_items[0] /= self.to_divide;
    }

    pub fn one_round(&mut self, receivers: &[Rc<RefCell<Monkey>>]) {
        let count = self.holding_items.len();
        for _ in 1..=count {
            self.inspect_head_item();
            self.throw_head_item_to(receivers);
        }
        self.inspections += count;
    }

    pub fn one_round2(&mut self, receivers: &[Rc<RefCell<Monkey>>], divisor: usize) {
        let count = self.holding_items.len();
        self.to_divide = 1;
        for _ in 1..=count {
            self.inspect_head_item();
            self.holding_items[0] %= divisor;
            self.throw_head_item_to(receivers);
        }
        self.inspections += count;
    }
}

#[derive(Debug, Clone)]
struct MonkeyGroup {
    monkeys: Vec<Rc<RefCell<Monkey>>>,
}

impl MonkeyGroup {
    pub fn new(monkeys: Vec<Rc<RefCell<Monkey>>>) -> Self {
        Self { monkeys }
    }

    pub fn one_round(&self) {
        self.monkeys
            .iter()
            .for_each(|monkey| monkey.borrow_mut().one_round(&self.monkeys));
    }

    pub fn one_round2(&self, divisor: usize) {
        self.monkeys
            .iter()
            .for_each(|monkey| monkey.borrow_mut().one_round2(&self.monkeys, divisor));
    }

    pub fn get_results(&self) -> Option<u64> {
        let mut v = self
            .monkeys
            .iter()
            .map(|m| std::cmp::Reverse(m.borrow().inspections))
            .collect::<Vec<_>>();

        v.sort();
        Some((v[0].0 * v[1].0) as u64)
    }

    pub fn get_divisor_product(&self) -> usize {
        self.monkeys
            .iter()
            .map(|mk| mk.borrow().divisiable)
            .product()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkeys = MonkeyGroup::new(
        input
            .split("\n\n")
            .map(|s| Rc::new(RefCell::new(monkey(s).unwrap())))
            .collect::<Vec<_>>(),
    );

    (0..20).for_each(|_| monkeys.one_round());
    monkeys.get_results()
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkeys = MonkeyGroup::new(
        input
            .split("\n\n")
            .map(|s| Rc::new(RefCell::new(monkey(s).unwrap())))
            .collect::<Vec<_>>(),
    );
    let divisor_product = monkeys.get_divisor_product();

    (0..10000).for_each(|_| monkeys.one_round2(divisor_product));
    monkeys.get_results()
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
