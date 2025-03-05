advent_of_code::solution!(5);
use itertools::Itertools;
use peg::parser;
use std::collections::VecDeque;

parser! {
    grammar crates() for str {
        pub rule root() -> Vec<VecDeque<char>>
        = r:(row()*) last_line() {
            let queue_count = r[0].len();
            let mut ret = vec![VecDeque::new(); queue_count];
            for row in r {
                for i in 0..row.len() {
                    if let Some(c) = row[i] {
                        ret[i].push_front(c);
                    }
                }
            }
            ret
        }

        rule row() -> Vec<Option<char>>
        = c:(col() ** " ") whitespaces() { c }

        rule col() -> Option<char>
        = e:(single() / empty()) { e }

        rule single() -> Option<char>
        = "[" ch:$(['a'..='z' | 'A'..='Z']) "]" { ch.chars().next() }

        rule empty() -> Option<char>
        = "   " { None }

        rule last_line()
        = ((" "*<0,1> ['0'..='9'] " ") ** " ")

        rule whitespaces()
        = ['\t' | '\n' | ' ']+

    }
}

struct Command {
    from: usize,
    to: usize,
    count: usize,
}

impl Command {
    pub fn new(from: usize, to: usize, count: usize) -> Self {
        Self { from, to, count }
    }

    pub fn execute(&self, queues: &mut Vec<VecDeque<char>>) {
        for _ in 0..self.count {
            if let Some(v) = &mut queues[self.from - 1].pop_back() {
                queues[self.to - 1].push_back(v.clone());
            } else {
                unreachable!()
            }
        }
    }

    pub fn execute_9001(&self, queues: &mut Vec<VecDeque<char>>) {
        let mut tmp = vec![];
        for _ in 0..self.count {
            if let Some(v) = &mut queues[self.from - 1].pop_back() {
                tmp.push(v.clone());
            } else {
                unreachable!()
            }
        }
        queues[self.to - 1].extend(tmp.into_iter().rev());
    }
}
// move 1 from 8 to 7
parser! {
    grammar command() for str {
        pub rule command() -> Command
        = "move " count:number() " from " from:number() " to " to:number() {
            Command::new(from, to, count)
        }

        pub rule number() -> usize
        = num:$(['0'..='9']+) { num.parse::<usize>().unwrap()}

        pub rule whitespaces()
        = [' ' | '\t' | '\n']+
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let crates = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .join("\n");

    let mut queues = crates::root(&crates).unwrap();
    lines
        .map(|line| command::command(line).unwrap())
        .for_each(|cmd| cmd.execute(&mut queues));

    let s = queues.into_iter().fold(String::new(), |mut s, mut q| {
        if let Some(c) = q.pop_back() {
            s.push(c);
        }
        s
    });
    println!("{s}");
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let crates = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .join("\n");

    let mut queues = crates::root(&crates).unwrap();
    lines
        .map(|line| command::command(line).unwrap())
        .for_each(|cmd| cmd.execute_9001(&mut queues));

    let s = queues.into_iter().fold(String::new(), |mut s, mut q| {
        if let Some(c) = q.pop_back() {
            s.push(c);
        }
        s
    });
    println!("{s}");
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
