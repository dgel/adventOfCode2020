use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use advent_of_code_2020::Mat;

use gumdrop::Options;

fn read_nums(filepath: &str) -> Vec<u64> {
    match File::open(filepath) {
        Ok(file) => std::io::BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| line.parse::<u64>().ok())
            .collect(),
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

fn distribution_of_difference(jolts: &[u64]) -> BTreeMap<u64, u64> {
    let mut jolts_sorted: Vec<u64> = jolts.to_vec();
    jolts_sorted.sort();
    jolts_sorted.insert(0, 0);
    jolts_sorted.push(jolts_sorted.last().unwrap() + 3);

    let mut counts = BTreeMap::new();
    for diff in jolts_sorted.windows(2).map(|slice| slice[1] - slice[0]) {
        *counts.entry(diff).or_insert(0) += 1;
    }
    counts
}

fn number_valid_arrangements(jolts: &[u64]) -> u64 {
    let mut jolts_sorted = jolts.to_vec();
    jolts_sorted.sort();
    jolts_sorted.push(jolts_sorted.last().unwrap() + 3);

    fn number_valid_arrangements_from(
        index: usize,
        previous_value: u64,
        jolts: &[u64],
        cache: &mut BTreeMap<(usize, u64), u64>,
    ) -> u64 {
        if index == jolts.len() - 1 {
            if jolts.last().unwrap() - previous_value <= 3 {
                return 1;
            } else {
                return 0;
            }
        }

        if let Some(result) = cache.get(&(index, previous_value)) {
            return *result;
        }
        let mut result = number_valid_arrangements_from(index + 1, jolts[index], jolts, cache);

        if jolts[index + 1] - previous_value <= 3 {
            result += number_valid_arrangements_from(index + 1, previous_value, jolts, cache);
        }
        cache.insert((index, previous_value), result);
        result
    }

    let mut cache = BTreeMap::new();
    number_valid_arrangements_from(0, 0, &jolts_sorted, &mut cache)
}

fn number_valid_arrangements_alternative(jolts: &[u64]) -> u64 {
    let mut jolts_sorted = jolts.to_vec();
    jolts_sorted.sort();
    let mut table = Mat::<u64>::new(jolts_sorted.len(), 3, 0);
    let last_item = jolts_sorted.len() - 1;
    table[(last_item, 0)] = 1;
    table[(last_item, 1)] = 1;
    table[(last_item, 2)] = 1;
    for i in (0..jolts_sorted.len() - 1).rev() {
        let diffnext = jolts_sorted[i + 1] - jolts_sorted[i];
        let count_keep = table[(i + 1, (diffnext - 1) as usize)];
        for j in 0..3 {
            let mut combined_count = count_keep;
            if j + diffnext < 3 {
                combined_count += table[(i + 1, (j + diffnext) as usize)];
            }
            table[(i, j as usize)] = combined_count;
        }
    }
    table[(0, (jolts_sorted[0] - 1) as usize)]
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let nums = read_nums(&opts.input_file);
    if !nums.is_empty() {
        let distribution = distribution_of_difference(&nums);
        let count1 = distribution.get(&1);
        let count3 = distribution.get(&3);
        if let (Some(c1), Some(c3)) = (count1, count3) {
            println!("Part 1: {}", c1 * c3);
        } else {
            println!(
                "Part 1: no counts for either difference of 1: {:?} or 3: {:?}",
                count1, count3
            );
        }
        println!("Part 2: {}", number_valid_arrangements(&nums));
        println!("Part 2 (alternative solution): {}", number_valid_arrangements_alternative(&nums));
    }
}
