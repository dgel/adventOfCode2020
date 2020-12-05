use std::io::prelude::*;
use std::fs::File;

use gumdrop::Options;

struct Ticket {
    row: u64,
    column: u64
}

impl Ticket {
    fn parse(line: &str) -> Option<Ticket> {
        let mut row = 0;
        let mut column = 0;
        for chr in line.chars() {
            match chr {
                'B' => {
                    row <<= 1;
                    row |= 1;
                },
                'F' => {
                    row <<= 1;
                },
                'R' => {
                    column <<= 1;
                    column |= 1;
                },
                'L' => {
                    column <<= 1;
                },
                _ => {
                    println!("found illegal character in ticket specification: '{}'", chr);
                    return None;
                }

            }
        }
        Some(Ticket{row, column})
    }
}

fn read_tickets(filepath: &str) -> Vec<Ticket> {
    match File::open(filepath) {
        Ok(file) => std::io::BufReader::new(file).lines().filter_map(Result::ok).filter_map(|a| Ticket::parse(&a)).collect(),
        Err(error) => {
            println!("could not open file '{}': {}", filepath, error);
            Vec::new()
        }
    } 
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String
}

fn highest_seat_id(tickets: &[Ticket]) -> u64 {
    tickets.iter().map(|t| t.row * 8 +  t.column).max().unwrap_or(0)
}

fn find_missing_seat_id(tickets: &[Ticket]) -> Option<u64> {
    let mut seat_ids: Vec<u64> = tickets.iter().map(|t| t.row * 8 + t.column).collect();
    seat_ids.sort();
    for sublist in seat_ids.windows(2) {
        if let &[left, right] = sublist {
            if left + 1 != right {
                return Some(left + 1);
            }
        }
    }
    None
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let tickets = read_tickets(&opts.input_file);
    if !tickets.is_empty() {
        println!("Part 1: {}", highest_seat_id(&tickets));
        if let Some(seat_id) = find_missing_seat_id(&tickets) {
            println!("Part 2: missing seat is number: {}", seat_id);
        } else {
            println!("Part 2: missing seat not found");
        }
    }
}
