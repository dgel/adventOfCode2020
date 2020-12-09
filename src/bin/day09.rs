use std::io::prelude::*;
use std::fs::File;

use gumdrop::Options;

fn read_nums(filepath: &str) -> Vec<i64> {
    match File::open(filepath) {
        Ok(file) => std::io::BufReader::new(file).lines().filter_map(|line| line.ok()).filter_map(|line| line.parse::<i64>().ok()).collect(),
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

fn is_valid_number(target: i64, preceding: &[i64]) -> bool {
    for (i, num) in preceding.iter().enumerate() {
        if preceding[i+1..].contains(&(target - num)) {
            return true;
        }
    }
    false
}

fn first_invalid_number(numbers: &[i64]) -> Option<i64> {
    let mut index: usize = 25; 
    while index < numbers.len() {
        if !is_valid_number(numbers[index], &numbers[index - 25 .. index]) {
            return Some(numbers[index]);
        }
        index += 1
    }
    None
}

fn subrange_sums_to(target: i64, numbers:&[i64]) -> Option<&[i64]> {
    let mut partial_sums = vec![0; numbers.len()];
    for (j, &num) in numbers.iter().enumerate() {
        partial_sums[j] = num;
        for (i, sum) in partial_sums[0..j].iter_mut().enumerate() {
            *sum += num;
            if *sum == target {
                return Some(&numbers[i..j]);
            }
        }
    }
    None
}



fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let nums = read_nums(&opts.input_file);
    if !nums.is_empty() {
        if let Some(value) = first_invalid_number(&nums) {
            println!("Part 1: {}", value);
            if let Some(range) = subrange_sums_to(value, &nums) {
                let min_opt = range.iter().min();
                let max_opt = range.iter().max();
                if let (Some(min), Some(max)) = (min_opt, max_opt) {
                    println!("Part 2: {}", min + max);
                } else {
                    println!("Part 2: range is empty");
                }
            } else {
                println!("Part 2: no subrange found that sums to {}", value);
            }
        } else {
            println!("Part 1: No result found");
        }
    }
}
