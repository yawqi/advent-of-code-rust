use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Passport<'a> {
    pub contents: Vec<&'a str>,
}

impl<'a> Passport<'a> {
    //  `byr` (birth year)
    //  `iyr` (issue year)
    //  `eyr` (expiration year)
    //  `hgt` (height)
    //  `hcl` (hair color)
    //  `ecl` (eye color)
    //  `pid` (passport id)
    fn is_valid(&self) -> bool {
        let mut map = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .map(|v| (v, false))
            .collect::<HashMap<_, _>>();

        self.contents.iter().for_each(|name| {
            if let Some((title, _)) = name.split_once(':') {
                let e = map.entry(title).or_default();
                *e = true;
            }
        });

        map.values().all(|v| *v)
    }
}

fn get_passport<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<Passport<'a>> {
    let mut contents = vec![];
    for line in iter {
        if line.trim().is_empty() {
            return Some(Passport { contents });
        }
        contents.extend(line.split_ascii_whitespace());
    }

    if contents.is_empty() {
        None
    } else {
        Some(Passport { contents })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut passports = vec![];
    while let Some(passport) = get_passport(&mut lines) {
        passports.push(passport);
    }
    Some(passports.into_iter().filter(|p| p.is_valid()).count() as u64)
}

#[derive(Debug, Clone, Copy)]
enum PassportField {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
}

use anyhow::{anyhow, Error, Ok};
use std::str::FromStr;

impl FromStr for PassportField {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (k, v) = s.split_once(':').ok_or(anyhow!("not enough"))?;
        match k {
            "byr" => {
                let value = v.parse::<u32>()?;
                if !(1920..=2002).contains(&value) {
                    return Err(anyhow!("invalid byr"));
                }
                Ok(PassportField::BYR)
            }
            "iyr" => {
                let value = v.parse::<u32>()?;
                if !(2010..=2020).contains(&value) {
                    return Err(anyhow!("invalid iyr"));
                }
                Ok(PassportField::IYR)
            }
            "eyr" => {
                let value = v.parse::<u32>()?;
                if !(2020..=2030).contains(&value) {
                    return Err(anyhow!("invalid eyr"));
                }
                Ok(PassportField::EYR)
            }
            "hgt" => {
                let bytes = v.as_bytes();
                let len = bytes.len();
                let mut num = 0;
                for b in &bytes[..len - 2] {
                    if !b.is_ascii_digit() {
                        return Err(anyhow!("invalid height"));
                    }
                    num *= 10;
                    num += (*b - b'0') as i32;
                }

                let unit = bytes[len - 2..]
                    .iter()
                    .map(|b| *b as char)
                    .collect::<String>();

                match unit {
                    s if s == "cm" => {
                        if !(150..=193).contains(&num) {
                            Err(anyhow!("invalid height in cm {num}"))
                        } else {
                            Ok(PassportField::HGT)
                        }
                    }
                    s if s == "in" => {
                        if !(59..=76).contains(&num) {
                            Err(anyhow!("invalid height in cm {num}"))
                        } else {
                            Ok(PassportField::HGT)
                        }
                    }
                    _ => Err(anyhow!("nope")),
                }
            }
            "hcl" => {
                if v.len() != 7 {
                    return Err(anyhow!("hcl invalid len"));
                }
                let bytes = v.as_bytes();
                if bytes[0] != b'#' {
                    return Err(anyhow!("hcl not sharp"));
                }

                if bytes[1..]
                    .iter()
                    .all(|c| char::is_ascii_hexdigit(&(*c as char)))
                {
                    Ok(PassportField::HCL)
                } else {
                    Err(anyhow!("invalid hcl"))
                }
            }

            "ecl" => match v {
                "blu" | "brn" | "gry" | "grn" | "amb" | "hzl" | "oth" => Ok(PassportField::ECL),
                _ => Err(anyhow!("invalid hair color")),
            },
            "pid" => {
                let mut count = 0;
                let res = v.chars().all(|ch| {
                    count += 1;
                    ch.is_ascii_digit()
                });

                if res && count == 9 {
                    Ok(PassportField::PID)
                } else {
                    Err(anyhow!("invalid pid"))
                }
            }
            "cid" => Ok(PassportField::CID),
            _ => Err(anyhow!("Not valid key {k}")),
        }
    }
}

impl<'a> Passport<'a> {
    fn is_valid_2(&self) -> bool {
        let mut map = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .map(|v| (v, None))
            .collect::<HashMap<_, _>>();

        self.contents.iter().for_each(|name| {
            if let Some((title, _)) = name.split_once(':') {
                let e = map.entry(title).or_default();
                *e = name.parse::<PassportField>().ok();
            }
        });

        if !map.values().all(|v| v.is_some()) {
            return false;
        }

        true
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut passports = vec![];
    while let Some(passport) = get_passport(lines.by_ref()) {
        passports.push(passport);
    }

    Some(passports.into_iter().filter(|p| p.is_valid_2()).count() as u64)
}

// * `byr` (Birth Year) - four digits; at least `1920` and at most `2002`.
// * `iyr` (Issue Year) - four digits; at least `2010` and at most `2020`.
// * `eyr` (Expiration Year) - four digits; at least `2020` and at most `2030`.
// * `hgt` (Height) - a number followed by either `cm` or `in`:
//   * If `cm`, the number must be at least `150` and at most `193`.
//   * If `in`, the number must be at least `59` and at most `76`.
//
// * `hcl` (Hair Color) - a `#` followed by exactly six characters `0`-`9` or `a`-`f`.
// * `ecl` (Eye Color) - exactly one of: `amb` `blu` `brn` `gry` `grn` `hzl` `oth`.
// * `pid` (Passport ID) - a nine-digit number, including leading zeroes.
// * `cid` (Country ID) - ignored, missing or not.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Year(u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Height {
    Cm(u64),
    In(u64),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct HairColor<'a>(&'a str);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct EyeColor<'a>(&'a str);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Id<'a>(&'a str);

struct PassportRefactored<'a> {
    byr: Year,
    iyr: Year,
    eyr: Year,
    hgt: Height,
    hcl: HairColor<'a>,
    ecl: EyeColor<'a>,
    pid: Id<'a>,
    cid: Option<Id<'a>>,
}

struct PassportBuilder<'a> {
    byr: Option<Year>,
    iyr: Option<Year>,
    eyr: Option<Year>,
    hgt: Option<Height>,
    hcl: Option<HairColor<'a>>,
    ecl: Option<EyeColor<'a>>,
    pid: Option<Id<'a>>,
    cid: Option<Id<'a>>,
}

impl<'a> PassportBuilder<'a> {
    pub fn build(self) -> Result<PassportRefactored<'a>, anyhow::Error> {
        macro_rules! build {
            (
                required => { $($id:ident),* }$(,)*
                optional => { $($opt:ident),* }$(,)*,
            ) => {
                Ok(PassportRefactored {
                    $($id: self.$id.ok_or(anyhow!("Missing filed {:?}", self.$id))?),*,
                    $($opt: self.$opt),*
                })
            }
        }

        build!(
            required => {byr, iyr, eyr, hgt, hcl, ecl, pid},
            optional => {cid},
        )
    }
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
