advent_of_code::solution!(7);

// $ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir {
    name: String,
    files: HashMap<String, File>,
    parent: Option<Rc<RefCell<Dir>>>,
    subdirs: HashMap<String, Rc<RefCell<Dir>>>,
    size: Option<u64>,
}

impl Dir {
    pub fn new(name: String) -> Self {
        Self {
            name,
            files: HashMap::new(),
            parent: None,
            subdirs: HashMap::new(),
            size: None,
        }
    }

    pub fn set_parent(&mut self, parent: Option<Rc<RefCell<Dir>>>) {
        self.parent = parent;
    }

    pub fn getsize(&mut self) -> u64 {
        if self.size.is_none() {
            self.size = Some(
                self.files
                    .values()
                    .fold(0, |sum, file| sum + file.getsize())
                    + self
                        .subdirs
                        .values()
                        .fold(0, |sum, dir| sum + dir.borrow_mut().getsize()),
            );
        }
        *self.size.as_ref().unwrap()
    }

    pub fn change_dir(&self, rpath: &str) -> Option<Rc<RefCell<Dir>>> {
        if let Some((curr, next)) = rpath.split_once('/') {
            self.subdirs
                .get(curr)
                .map(|next_dir| next_dir.borrow().change_dir(next).unwrap())
        } else if rpath.starts_with("..") {
            self.parent.clone()
        } else {
            self.subdirs.get(rpath).cloned()
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn fill_dir(&mut self, output: &str, parent_ptr: Option<Rc<RefCell<Self>>>) {
        let lines = output.lines();
        lines
            .clone()
            .filter(|s| !s.starts_with("dir"))
            .flat_map(|line| line.parse::<File>())
            .for_each(|file| {
                self.files.insert(file.name(), file);
            });

        lines
            .filter(|s| s.starts_with("dir"))
            .map(|line| {
                let mut cmd = line.parse::<Dir>().unwrap();
                cmd.set_parent(parent_ptr.clone());
                cmd
            })
            .for_each(|dir| {
                self.subdirs.insert(dir.name(), Rc::new(RefCell::new(dir)));
            });
    }

    pub fn get_dir_count(&mut self) -> u64 {
        let mut count = 0;
        if self.getsize() <= 100000 {
            count += self.getsize();
        }

        count
            + self.subdirs.values().fold(0, |count, subdir| {
                count + subdir.borrow_mut().get_dir_count()
            })
    }

    pub fn get_sizes(&mut self, sizes: &mut Vec<u64>) {
        sizes.push(self.getsize());

        self.subdirs.values().for_each(|subdir| {
            subdir.borrow_mut().get_sizes(sizes);
        })
    }
}

impl FromStr for Dir {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, name) = s.split_once(' ').ok_or(anyhow!("not valid dir format"))?;
        Ok(Self::new(name.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    name: String,
    size: u64,
}

impl File {
    pub fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
    pub fn getsize(&self) -> u64 {
        self.size
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl FromStr for File {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, name) = s.split_once(' ').ok_or(anyhow!("not valid file format"))?;
        let size = size.parse::<u64>()?;
        Ok(File::new(name.to_owned(), size))
    }
}

enum Command {
    Cd(String),
    Ls(String),
}

impl FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("ls") {
            Ok(Self::Ls(s.lines().skip(1).join("\n")))
        } else {
            let (_, path) = s
                .split_once(' ')
                .ok_or(anyhow!("not valid cd format s:{} ", s))?;
            Ok(Self::Cd(path.trim().to_owned()))
        }
    }
}

impl Command {
    pub fn execute(self, prog: &mut Program) -> Result<()> {
        match self {
            Self::Cd(path) => prog.change_dir(&path),
            Self::Ls(out) => prog.list_dir(&out),
        }
    }
}

struct Program {
    root: Rc<RefCell<Dir>>,
    cwd: Rc<RefCell<Dir>>,
}

impl Program {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Dir::new("/".to_owned())));
        Program {
            cwd: root.clone(),
            root,
        }
    }

    pub fn change_dir(&mut self, dest: &str) -> Result<()> {
        match dest {
            dest if dest.starts_with("/") => {
                self.cwd = self.root.clone();
            }
            _ => {
                let next_cwd = self
                    .cwd
                    .borrow()
                    .change_dir(dest)
                    .ok_or(anyhow!("chang dir to {dest} failed"))?;
                self.cwd = next_cwd;
            }
        }
        Ok(())
    }

    pub fn list_dir(&mut self, output: &str) -> Result<()> {
        self.cwd
            .borrow_mut()
            .fill_dir(output, Some(self.cwd.clone()));
        Ok(())
    }

    pub fn get_results(&mut self) -> Option<u64> {
        Some(self.root.borrow_mut().get_dir_count())
    }

    pub fn get_sizes(&mut self) -> Option<u64> {
        let mut sizes = vec![];
        self.root.borrow_mut().get_sizes(&mut sizes);
        sizes.sort_by(|a, b| b.cmp(a));
        dbg!(&sizes);
        let target = sizes[0] - 40000000;
        sizes.into_iter().take_while(|v| *v >= target).last()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut prog = Program::new();
    input
        .split("$ ")
        .flat_map(|v| {
            let v = v.trim();
            if v.is_empty() {
                None
            } else {
                Some(v.parse::<Command>().unwrap())
            }
        })
        .for_each(|cmd| {
            cmd.execute(&mut prog).unwrap();
        });

    prog.get_results()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut prog = Program::new();
    input
        .split("$ ")
        .flat_map(|v| {
            let v = v.trim();
            if v.is_empty() {
                None
            } else {
                Some(v.parse::<Command>().unwrap())
            }
        })
        .for_each(|cmd| {
            cmd.execute(&mut prog).unwrap();
        });

    prog.get_sizes()
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
