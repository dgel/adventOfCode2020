use std::io::prelude::*;
use std::fs::File;

use gumdrop::Options;

fn read_map(filepath: &str) -> Vec<Vec<u8>> {
    match File::open(filepath) {
        Ok(file) => std::io::BufReader::new(file).lines().filter_map(|line| line.ok().map(|l| l.bytes().collect())).collect(),
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

fn num_trees_with_slope(map: &[Vec<u8>], right_step: usize, down_step: usize) -> usize {
    let mut count = 0;

    let mut pos = (0,0);
    let length = map.len();
    if length > 0 {
        let width = map[0].len();
        loop {
            if pos.0 >= length {
                break;
            }
            pos.1 = pos.1 % width;

            if map[pos.0][pos.1] == '#' as u8 {
                count += 1
            }

            pos = (pos.0 + down_step, pos.1 + right_step);
        }
    }

    count
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let map = read_map(&opts.input_file);
    if !map.is_empty() {
        println!("Part 1: {}", num_trees_with_slope(&map, 3, 1));
        let part2 = num_trees_with_slope(&map, 1, 1) 
                  * num_trees_with_slope(&map, 3 ,1)
                  * num_trees_with_slope(&map, 5 ,1)
                  * num_trees_with_slope(&map, 7 ,1)
                  * num_trees_with_slope(&map, 1 ,2);
        println!("Part 2: {}", part2);

    }
}
