use std::fs::File;
use std::io::prelude::*;

use divrem::DivRemEuclid;
use gumdrop::Options;

fn read_schedule(filepath: &str) -> Option<(i128, Vec<Option<i128>>)> {
    match File::open(filepath) {
        Ok(file) => {
            let mut reader = std::io::BufReader::new(file);
            let mut line = String::new();
            if reader.read_line(&mut line).is_ok() {
                if let Some(time) = line.trim().parse().ok() {
                    line.clear();
                    if reader.read_line(&mut line).is_ok() {
                        let bustimes: Vec<Option<i128>> = line
                            .trim()
                            .split(',')
                            .map(|l| l.trim().parse::<i128>().ok())
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

fn next_after(time: i128, interval: i128) -> i128 {
    let diff = time % interval;
    time - diff + interval
}

fn combine_period_offset(
    period_a: i128,
    offset_a: i128,
    period_b: i128,
    offset_b: i128,
) -> Option<(i128, i128)> {
    let (gcd, s, _) = extended_gcd(period_a, period_b);

    let period_combined = period_a / gcd * period_b;
    let (quot, modulo) = (offset_b - offset_a).div_rem_euclid(gcd);
    if modulo != 0 {
        return None;
    }
    let num_periods = s * quot;
    let (_, offset) = (-num_periods * period_a - offset_a).div_rem_euclid(period_combined);
    Some((period_combined, -offset))
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let (quot, modulo) = old_r.div_rem_euclid(r);
        old_r = r;
        r = modulo;
        let mut tmp = s;
        s = old_s - quot * s;
        old_s = tmp;
        tmp = t;
        t = old_t - quot * t;
        old_t = tmp;
    }

    (old_r, old_s, old_t)
}

fn first_time_offsets_match(schedule: &[Option<i128>]) -> Option<i128> {
        schedule
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(i, v)| v.map(|bi| (i, bi)))
            .fold(Some((1, 0)), |acc, (offset_a, period_a)| {
                acc.and_then(|(period_combined, offset_combined)| {
                    combine_period_offset(
                        period_a,
                        offset_a as i128,
                        period_combined,
                        offset_combined,
                    )
                })
            })
            .map(|(interval, offset)| interval + offset - schedule.len() as i128 + 1)
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let schedule = read_schedule(&opts.input_file);
    if let Some((time, schedule)) = schedule {
        if let Some((bus, arrival_time)) = schedule
            .iter()
            .filter_map(|&bus_interval| bus_interval.map(|bi| (bi, next_after(time, bi))))
            .min_by_key(|&(_, arrival_time)| arrival_time)
        {
            println!("Part 1: {}", bus * (arrival_time - time));
        } else {
            println!("Part 1: No result");
        }
        if let Some(time) = first_time_offsets_match(&schedule)
        {
            println!("Part 1: {}", time);
        } else {
            println!("Part 2: No result");
        }
    } else {
        println!("Had some issue parsing the input");
    }
}
