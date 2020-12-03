use std::io::prelude::*;
use std::collections::BTreeSet;
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

fn multiply_2_elements_sum_to(nums: &[i64], expected_sum: i64) -> Option<i64> {
    let elements: BTreeSet<i64> = nums.iter().map(|x| *x).collect();
    for num in nums {
        let target = expected_sum - num;
        if elements.contains(&target) {
            return Some(target * num);
        }
    }
    None
}

fn multiply_3_elements_sum_to(nums: &[i64], expected_sum: i64) -> Option<i64> {
    for i in 0..nums.len() - 2 {
        let num = nums[i];
        let new_target = expected_sum - num;
        if let Some(product) = multiply_2_elements_sum_to(&nums[(i+1)..nums.len()], new_target) {
            return Some(num * product);
        }
    }
    None
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let nums = read_nums(&opts.input_file);
    if let Some(num) = multiply_2_elements_sum_to(&nums, 2020) {
        println!("part 1: {}", num);
    } else {
        println!("part 1: result not found")
    }
    if let Some(num) = multiply_3_elements_sum_to(&nums, 2020) {
        println!("part 2: {}", num);
    } else {
        println!("part 2: result not found");
    }
}
