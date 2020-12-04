use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

use gumdrop::Options;
use regex::Regex;

fn has_match(regex: &Regex, passport: &BTreeMap<String, String>, field: &str) -> bool {
    if let Some(value) = passport.get(field) {
        regex.is_match(value)
    } else {
        false
    }
}

struct Validator {
    num: Regex,
    hgt: Regex,
    hcl: Regex,
    ecl: Regex,
    pid: Regex,
}

impl Validator {
    fn new() -> Self {
        Validator {
            num: Regex::new(r"^\d\d\d\d$").unwrap(),
            hgt: Regex::new(r"^(\d\d\d?)(in|cm)$").unwrap(),
            hcl: Regex::new(r"^#[0-9a-f]{6}$").unwrap(),
            ecl: Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
            pid: Regex::new(r"^\d{9}$").unwrap(),
        }
    }

    fn match_year(
        &self,
        passport: &BTreeMap<String, String>,
        field: &str,
        min_year: u32,
        max_year: u32,
    ) -> bool {
        if let Some(value) = passport.get(field) {
            if self.num.is_match(value) {
                let num: u32 = value.parse().unwrap();
                num >= min_year && num <= max_year
            } else {
                false
            }
        } else {
            false
        }
    }

    fn valid_height(&self, passport: &BTreeMap<String, String>) -> bool {
        if let Some(value) = passport.get("hgt") {
            if let Some(captures) = self.hgt.captures(value) {
                let num: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
                if captures.get(2).unwrap().as_str() == "cm" {
                    num >= 150 && num <= 193
                } else {
                    num >= 59 && num <= 76
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_valid_passport(&self, passport: &BTreeMap<String, String>) -> bool {
        self.match_year(passport, "byr", 1920, 2002)
            && self.match_year(passport, "iyr", 2010, 2020)
            && self.match_year(passport, "eyr", 2020, 2030)
            && has_match(&self.hcl, passport, "hcl")
            && has_match(&self.ecl, passport, "ecl")
            && has_match(&self.pid, passport, "pid")
            && self.valid_height(passport)
    }
}

fn read_passports(filepath: &str) -> Vec<BTreeMap<String, String>> {
    match File::open(filepath) {
        Ok(mut file) => {
            let mut input = String::new();
            if file.read_to_string(&mut input).is_ok() {
                input
                    .split("\n\n")
                    .map(|line| {
                        line.split_whitespace()
                            .filter_map(|item| {
                                let items = item.split(':').collect::<Vec<_>>();
                                if items.len() == 2 {
                                    Some((String::from(items[0]), String::from(items[1])))
                                } else {
                                    None
                                }
                            })
                            .collect()
                    })
                    .collect()
            } else {
                Vec::new()
            }
        }
        Err(error) => {
            println!("could not open file '{}': {}", filepath, error);
            Vec::new()
        }
    }
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn is_valid_passport(passport: &BTreeMap<String, String>) -> bool {
    passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid"))
}

fn num_valid_passports(passports: &[BTreeMap<String, String>]) -> usize {
    passports.iter().filter(|p| is_valid_passport(&p)).count()
}

fn num_valid_passports_strict(validator: &Validator, passports: &[BTreeMap<String, String>]) -> usize {
    passports.iter().filter(|p| validator.is_valid_passport(p)).count()
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let passports = read_passports(&opts.input_file);
    if !passports.is_empty() {
        println!("Part 1: {}", num_valid_passports(&passports));
        let validator = Validator::new();
        println!("Part 2: {}", num_valid_passports_strict(&validator, &passports));
    }
}
