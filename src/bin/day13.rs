use std::fs::File;
use std::io::prelude::*;

use gumdrop::Options;

fn read_schedule(filepath: &str) -> Option<(u64, Vec<u64>)> {
    match File::open(filepath) {
        Ok(file) => {
            let mut reader = std::io::BufReader::new(file);
            let mut line = String::new();
            if reader.read_line(&mut line).is_ok() {
                if let Some(time) = line.trim().parse().ok() {
                    line.clear();
                    if reader.read_line(&mut line).is_ok() {
                        println!("schedule line: '{}'", line);
                        let bustimes: Vec<u64> = line
                            .trim()
                            .split(',')
                            .filter_map(|l| l.trim().parse::<u64>().ok())
                            .collect();
                        return Some((time, bustimes));
                    } else {
                        println!("Could not read line 2");
                    }
                } else {
                    println!("Could not parse line '{}'", line);
                }
            }
            return None;
        }
        Err(error) => {
            println!("could not open file '{}': {}", filepath, error);
            None
        }
    }
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn next_after(time: u64, interval: u64) -> u64 {
    let diff = time % interval;
    time - diff + interval
}

fn first_bus(time: u64, intervals: &[u64]) -> Option<(u64, u64)> {
    intervals
        .iter()
        .map(|&bus_interval| (bus_interval, next_after(time, bus_interval)))
        .min_by_key(|&(_, arrival_time)| arrival_time)
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let schedule = read_schedule(&opts.input_file);
    if let Some((time, schedule)) = schedule {
        println!("{:?}", schedule);
        if let Some((bus, arrival_time)) = first_bus(time, &schedule) {
            println!("Part 1: {}", bus * (arrival_time - time));
        } else {
            println!("Part 1: No result");
        }
    } else {
        println!("Had some issue parsing the input");
    }
}
