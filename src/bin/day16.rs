use advent_of_code_2020::Mat;
use anyhow::{anyhow, Result};
use scan_fmt::*;
use std::fs::File;
use std::io::prelude::*;

use gumdrop::Options;

#[derive(Debug, Clone)]
struct Constraint {
    name: String,
    range1: (u16, u16),
    range2: (u16, u16),
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u16>,
}

impl Ticket {
    fn parse(line: &str) -> Result<Self> {
        Ok(Self {
            values: line
                .split(",")
                .map(|num| num.parse::<u16>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug)]
struct Input {
    constraints: Vec<Constraint>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn read_input(filepath: &str) -> Result<Input> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut parts = contents.split("\n\n");
    let constraint_part = parts.next().ok_or(anyhow!("missing constraints section"))?;
    let constraints = constraint_part
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let name = parts.next().ok_or(anyhow!("missing name part"))?;
            let ranges = parts
                .next()
                .ok_or(anyhow!("missing first constraint range"))?;
            let (r11, r12, r21, r22) =
                scan_fmt!(ranges, " {d}-{d} or {d}-{d}", u16, u16, u16, u16)?;
            Ok(Constraint {
                name: name.into(),
                range1: (r11, r12),
                range2: (r21, r22),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let my_ticket_part = parts.next().ok_or(anyhow!("missing own ticket section"))?;
    let my_ticket_line = my_ticket_part
        .lines()
        .skip(1)
        .next()
        .ok_or(anyhow!("missing own ticket"))?;
    let my_ticket = Ticket::parse(my_ticket_line)?;
    let nearby_tickets = parts
        .next()
        .ok_or(anyhow!("missing nearby ticket section"))?;
    let nearby_tickets = nearby_tickets
        .lines()
        .skip(1)
        .map(|line| Ticket::parse(line))
        .collect::<Result<Vec<_>>>()?;

    Ok(Input {
        constraints,
        my_ticket,
        nearby_tickets,
    })
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn number_matches(number: u16, constraint: &Constraint) -> bool {
    (number >= constraint.range1.0 && number <= constraint.range1.1)
        || (number >= constraint.range2.0 && number <= constraint.range2.1)
}

fn sum_invalid_values(constraints: &[Constraint], tickets: &[Ticket]) -> u16 {
    tickets
        .iter()
        .map(|ticket| {
            ticket
                .values
                .iter()
                .filter(|number| {
                    constraints
                        .iter()
                        .all(|constraint| !number_matches(**number, constraint))
                })
                .sum::<u16>()
        })
        .sum()
}

fn is_valid_ticket(constraints: &[Constraint], ticket: &Ticket) -> bool {
    ticket.values.iter().all(|number| {
        constraints
            .iter()
            .any(|constraint| number_matches(*number, constraint))
    })
}

fn fix_one(matches: &mut Mat<bool>, fixed: &mut Vec<bool>) -> bool {
    for constraint in 0..matches.height() {
        if !fixed[constraint] {
            let mut num_match = 0;
            let mut last_pos_match = 0;
            for position in 0..matches.width() {
                if matches[(constraint, position)] {
                    num_match += 1;
                    last_pos_match = position;
                }
            }
            if num_match == 1 {
                fixed[constraint] = true;
                for other_constraint in 0..matches.height() {
                    if other_constraint != constraint {
                        matches[(other_constraint, last_pos_match)] = false;
                    }
                }
                return true;
            }
        }
    }
    false
}

fn match_labels(constraints: &[Constraint], tickets: &[Ticket]) -> Vec<(String, usize)> {
    let valid_tickets = tickets
        .iter()
        .filter(|ticket| is_valid_ticket(constraints, ticket))
        .collect::<Vec<_>>();
    let mut matches = Mat::new(constraints.len(), constraints.len(), false);
    for constraint in 0..constraints.len() {
        for position in 0..constraints.len() {
            matches[(constraint, position)] = valid_tickets
                .iter()
                .all(|ticket| number_matches(ticket.values[position], &constraints[constraint]));
        }
    }

    let mut fixed = vec![false; matches.height()];
    while fix_one(&mut matches, &mut fixed) {
    }
    if matches.iter_elements().filter(|&&v| v).count() == matches.width() {
        (0..constraints.len()).map(|i| {
            let position = (0..constraints.len()).find(|&j| matches[(i, j)]).unwrap();
            (constraints[i].name.clone(), position)
        }).collect()
    } else {
        Vec::new()
    }
   }

fn main() -> Result<()> {
    let opts = Arguments::parse_args_default_or_exit();
    let input = read_input(&opts.input_file)?;
    println!(
        "Part 1: {}",
        sum_invalid_values(&input.constraints, &input.nearby_tickets)
    );
    let part2_result = match_labels(&input.constraints, &input.nearby_tickets);
    if !part2_result.is_empty() {
        let product: u64 = part2_result
            .iter()
            .map(|(label, index)| {
                if label.starts_with("departure") {
                    input.my_ticket.values[*index] as u64
                } else {
                    1
                }
            })
            .product();
        println!("Part 2: {}", product);
    } else {
        println!("Part 2: no result found");
    }
    Ok(())
}
