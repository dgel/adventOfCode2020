use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

use gumdrop::Options;
use scan_fmt::scan_fmt;

#[derive(Clone, Copy)]
struct BitMask {
    zeros: u64,
    ones: u64,
}

enum Instruction {
    SetBitmask(BitMask, Vec<BitMask>),
    Write { address: u64, value: u64 },
}

impl Instruction {
    fn parse(line: &str) -> Option<Instruction> {
        if line.starts_with("mask") {
            if let Some(mask) = line.split_whitespace().last() {
                let mut bitmask = BitMask { zeros: 0, ones: 0 };
                let mut floating_bitmasks = vec![BitMask { zeros: 0, ones: 0 }];

                for bit in mask.chars() {
                    bitmask.ones <<= 1;
                    bitmask.zeros <<= 1;
                    for mask in floating_bitmasks.iter_mut() {
                        mask.ones <<= 1;
                        mask.zeros <<= 1;
                    }
                    match bit {
                        '1' => {
                            bitmask.ones |= 1;
                            for mask in floating_bitmasks.iter_mut() {
                                mask.ones |= 1;
                            }
                        }
                        '0' => bitmask.zeros |= 1,
                        'X' => {
                            let mut new_masks = Vec::new();
                            for mask in floating_bitmasks {
                                let mut mask_one = mask;
                                mask_one.ones |= 1;
                                new_masks.push(mask_one);
                                let mut mask_zero = mask;
                                mask_zero.zeros |= 1;
                                new_masks.push(mask_zero);
                            }
                            floating_bitmasks = new_masks;
                        }
                        _ => (),
                    }
                }
                bitmask.zeros = !bitmask.zeros;
                for mask in floating_bitmasks.iter_mut() {
                    mask.zeros = !mask.zeros;
                }
                return Some(Instruction::SetBitmask(bitmask, floating_bitmasks));
            }
        } else if line.starts_with("mem") {
            return scan_fmt!(line, "mem[{d}] = {d}", u64, u64)
                .ok()
                .map(|(address, value)| Instruction::Write { address, value });
        }
        return None;
    }
}

fn read_instructions(filepath: &str) -> Vec<Instruction> {
    match File::open(filepath) {
        Ok(file) => std::io::BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| Instruction::parse(&line))
            .collect(),
        Err(error) => {
            println!("could not open file '{}': {}", filepath, error);
            Vec::new()
        }
    }
}

fn run_program1(instrs: &[Instruction]) -> BTreeMap<u64, u64> {
    let mut result = BTreeMap::new();
    let mut mask = BitMask { ones: 0, zeros: !0 };
    for instr in instrs {
        match instr {
            Instruction::SetBitmask(bitmask, _) => mask = *bitmask,
            Instruction::Write { address, value } => {
                result.insert(*address, (value | mask.ones) & mask.zeros);
            }
        }
    }
    result
}

fn run_program2(instrs: &[Instruction]) -> BTreeMap<u64, u64> {
    let mut result = BTreeMap::new();
    let mut masks = vec![BitMask { ones: 0, zeros: !0 }];
    for instr in instrs {
        match instr {
            Instruction::SetBitmask(_, new_masks) => masks = new_masks.clone(),
            Instruction::Write { address, value } => {
                for mask in masks.iter() {
                    result.insert((address | mask.ones) & mask.zeros, *value);
                }
            }
        }
    }
    result
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let instructions = read_instructions(&opts.input_file);
    if !instructions.is_empty() {
        println!(
            "Part 1: {}",
            run_program1(&instructions).values().sum::<u64>()
        );
        println!(
            "Part 2: {}",
            run_program2(&instructions).values().sum::<u64>()
        );
    }
}
